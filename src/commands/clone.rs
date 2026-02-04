use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::Path;

use crate::storage::S3Storage;
use crate::utils::validate_project_name;

pub async fn run(project_name: &str, current_dir: &Path) -> Result<()> {
    validate_project_name(project_name)?;

    let target_dir = current_dir.join(project_name);

    if target_dir.exists() {
        anyhow::bail!("ディレクトリが既に存在します: {}", target_dir.display());
    }

    let storage = S3Storage::new(None).await?;
    let history = storage
        .get_history(project_name)
        .await?
        .ok_or_else(|| anyhow::anyhow!("プロジェクト '{}' が見つかりません", project_name))?;

    let snapshot = history
        .snapshots
        .last()
        .ok_or_else(|| anyhow::anyhow!("スナップショットがありません"))?;

    println!("クローン中: {}", project_name);
    println!("スナップショット: {}", snapshot.id);
    if let Some(msg) = &snapshot.message {
        println!("メッセージ: {}", msg);
    }
    println!("ファイル数: {}\n", snapshot.files.len());

    fs::create_dir_all(&target_dir)?;

    let pb = ProgressBar::new(snapshot.files.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len}")
            .expect("プログレスバーのテンプレートエラー")
            .progress_chars("#>-"),
    );

    for (relative_path, hash) in &snapshot.files {
        let target_path = target_dir.join(relative_path);

        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let data = storage.download_blob(project_name, hash).await?;
        fs::write(&target_path, data)?;

        pb.inc(1);
    }

    pb.finish_with_message("クローン完了");

    let gp_dir = target_dir.join(".gp");
    fs::create_dir_all(&gp_dir)?;

    println!("\nクローン完了: {}", target_dir.display());

    Ok(())
}
