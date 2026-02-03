mod cli;
mod error;
mod scanner;
mod storage;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use scanner::{diff_files, Scanner};
use storage::{extract_project_name, S3Storage};
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Push {
            path,
            message,
            dry_run,
        } => {
            push_command(&path, message.as_deref(), dry_run).await?;
        }
        Commands::Log { project, limit } => {
            log_command(project.as_deref(), limit).await?;
        }
        Commands::Checkout { snapshot, output } => {
            checkout_command(&snapshot, output.as_deref()).await?;
        }
        Commands::Init { path } => {
            init_command(&path)?;
        }
        Commands::Status { path } => {
            status_command(&path).await?;
        }
    }

    Ok(())
}

async fn push_command(path: &Path, message: Option<&str>, dry_run: bool) -> Result<()> {
    let path = fs::canonicalize(path)?;
    let project_name = extract_project_name(&path);

    println!("プロジェクト: {}", project_name);
    println!("スキャン中...");

    let scanner = Scanner::new(&path)?;
    let local_files = scanner.scan()?;

    println!("ファイル数: {}", local_files.len());

    let storage = S3Storage::new(None).await?;
    let remote_state = storage.get_remote_state(&project_name).await?;

    let changed_files = diff_files(&local_files, &remote_state);

    if changed_files.is_empty() {
        println!("変更されたファイルはありません");
        return Ok(());
    }

    println!("変更ファイル数: {}", changed_files.len());

    if dry_run {
        println!("\n[ドライラン] アップロード予定のファイル:");
        for file in &changed_files {
            println!("  {}", file.relative_path.display());
        }
        return Ok(());
    }

    storage.upload_files(&project_name, &changed_files).await?;
    storage.save_state(&project_name, &local_files).await?;

    if let Some(msg) = message {
        println!("\nメッセージ: {}", msg);
    }

    println!("\nプッシュ完了: s3://{}/{}/", storage.bucket(), project_name);

    Ok(())
}

async fn log_command(_project: Option<&str>, _limit: usize) -> Result<()> {
    println!("log コマンドはフェーズ2で実装予定です");
    Ok(())
}

async fn checkout_command(_snapshot: &str, _output: Option<&Path>) -> Result<()> {
    println!("checkout コマンドはフェーズ2で実装予定です");
    Ok(())
}

fn init_command(path: &Path) -> Result<()> {
    let gp_dir = path.join(".gp");
    if !gp_dir.exists() {
        fs::create_dir_all(&gp_dir)?;
    }

    let ignore_file = path.join(".gp-ignore");
    if !ignore_file.exists() {
        let default_content = r#"# GroovePush 除外設定
# DAWの一時ファイル等を除外

# Ableton Live
*.tmp
Backup/
*.asd

# Logic Pro
*.autosave

# FL Studio
*.flpbackup

# 一般的な一時ファイル
.DS_Store
Thumbs.db
*.bak
"#;
        fs::write(&ignore_file, default_content)?;
    }

    println!("GroovePush を初期化しました: {}", path.display());
    Ok(())
}

async fn status_command(path: &Path) -> Result<()> {
    let path = fs::canonicalize(path)?;
    let project_name = extract_project_name(&path);

    println!("プロジェクト: {}", project_name);

    let scanner = Scanner::new(&path)?;
    let local_files = scanner.scan()?;

    println!("ローカルファイル数: {}", local_files.len());

    let total_size: u64 = local_files.iter().map(|f| f.size).sum();
    println!("合計サイズ: {}", format_size(total_size));

    let storage = S3Storage::new(None).await?;
    let remote_state = storage.get_remote_state(&project_name).await?;

    if remote_state.is_empty() {
        println!("リモート: まだプッシュされていません");
    } else {
        let changed_files = diff_files(&local_files, &remote_state);
        println!("変更ファイル数: {}", changed_files.len());
    }

    Ok(())
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
