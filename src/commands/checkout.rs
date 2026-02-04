use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::Path;

use crate::storage::{extract_project_name, S3Storage};

pub async fn run(snapshot_id: &str, output: Option<&Path>) -> Result<()> {
    let path = match output {
        Some(p) => p.to_path_buf(),
        None => std::env::current_dir()?,
    };

    let project_name = extract_project_name(&path);

    let storage = S3Storage::new(None).await?;
    let history = storage
        .get_history(&project_name)
        .await?
        .ok_or_else(|| {
            anyhow::anyhow!("プロジェクト '{}' の履歴が見つかりません", project_name)
        })?;

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
