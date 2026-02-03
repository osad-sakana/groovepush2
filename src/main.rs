mod cli;
mod error;
mod scanner;
mod storage;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use indicatif::{ProgressBar, ProgressStyle};
use scanner::{diff_files, Scanner};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use storage::{extract_project_name, History, S3Storage, Snapshot};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let current_dir = std::env::current_dir()?;

    match cli.command {
        Commands::Push { message, dry_run } => {
            push_command(&current_dir, message.as_deref(), dry_run).await?;
        }
        Commands::Log { project, limit } => {
            log_command(project.as_deref(), limit).await?;
        }
        Commands::Checkout { snapshot, output } => {
            checkout_command(&snapshot, output.as_deref()).await?;
        }
        Commands::Init => {
            init_command(&current_dir)?;
        }
        Commands::Status => {
            status_command(&current_dir).await?;
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

    println!("blobsにアップロード中...");
    let new_blobs = storage.upload_blobs(&project_name, &changed_files).await?;
    println!("新規blob: {} 件", new_blobs);

    println!("ファイルをアップロード中...");
    storage.upload_files(&project_name, &changed_files).await?;
    storage.save_state(&project_name, &local_files).await?;

    let mut history = storage
        .get_history(&project_name)
        .await?
        .unwrap_or_else(|| History::new(&project_name));

    let parent_id = history.head.clone();

    let files_map: HashMap<String, String> = local_files
        .iter()
        .map(|f| {
            (
                f.relative_path.to_string_lossy().to_string(),
                f.hash.clone(),
            )
        })
        .collect();

    let total_size: u64 = local_files.iter().map(|f| f.size).sum();

    let snapshot = Snapshot::new(
        message.map(String::from),
        files_map,
        parent_id,
        total_size,
        changed_files.len(),
    );

    let snapshot_id = snapshot.id.clone();
    history.add_snapshot(snapshot);

    storage.save_history(&project_name, &history).await?;

    println!("\nスナップショット: {}", snapshot_id);
    if let Some(msg) = message {
        println!("メッセージ: {}", msg);
    }

    println!("プッシュ完了: s3://{}/{}/", storage.bucket(), project_name);

    Ok(())
}

async fn log_command(project: Option<&str>, limit: usize) -> Result<()> {
    let project_name = match project {
        Some(p) => p.to_string(),
        None => {
            let path = fs::canonicalize(".")?;
            extract_project_name(&path)
        }
    };

    let storage = S3Storage::new(None).await?;
    let history = storage.get_history(&project_name).await?;

    match history {
        None => {
            println!("プロジェクト '{}' の履歴が見つかりません", project_name);
            println!("まず 'gp push' でプッシュしてください");
        }
        Some(h) => {
            println!("プロジェクト: {}\n", h.project_name);

            if h.snapshots.is_empty() {
                println!("スナップショットはありません");
                return Ok(());
            }

            let total = h.snapshots.len();
            for snapshot in h.snapshots.iter().rev().take(limit) {
                println!("snapshot {}", snapshot.id);
                if let Some(msg) = &snapshot.message {
                    println!("メッセージ: {}", msg);
                }
                println!(
                    "日時: {}",
                    snapshot.created_at.format("%Y-%m-%d %H:%M:%S UTC")
                );
                println!(
                    "ファイル数: {} (変更: {})",
                    snapshot.meta.file_count, snapshot.meta.changed_count
                );
                println!("サイズ: {}\n", format_size(snapshot.meta.total_size));
            }

            let shown = limit.min(total);
            println!("(全{}件中{}件表示)", total, shown);
        }
    }

    Ok(())
}

async fn checkout_command(snapshot_id: &str, output: Option<&Path>) -> Result<()> {
    let path = match output {
        Some(p) => p.to_path_buf(),
        None => std::env::current_dir()?,
    };

    let project_name = extract_project_name(&path);

    let storage = S3Storage::new(None).await?;
    let history = storage
        .get_history(&project_name)
        .await?
        .ok_or_else(|| anyhow::anyhow!("プロジェクト '{}' の履歴が見つかりません", project_name))?;

    let snapshot = history
        .find_snapshot_by_prefix(snapshot_id)
        .ok_or_else(|| {
            anyhow::anyhow!("スナップショットが見つかりません: {}", snapshot_id)
        })?;

    println!("復元中: {}", snapshot.id);
    if let Some(msg) = &snapshot.message {
        println!("メッセージ: {}", msg);
    }
    println!("ファイル数: {}\n", snapshot.files.len());

    let pb = ProgressBar::new(snapshot.files.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len}")
            .expect("プログレスバーのテンプレートエラー")
            .progress_chars("#>-"),
    );

    for (relative_path, hash) in &snapshot.files {
        let target_path = path.join(relative_path);

        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let data = storage.download_blob(&project_name, hash).await?;
        fs::write(&target_path, data)?;

        pb.inc(1);
    }

    pb.finish_with_message("復元完了");

    println!("\n復元完了: {}", snapshot.id);
    println!("ディレクトリ: {}", path.display());

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
