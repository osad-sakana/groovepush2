use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::scanner::{diff_files, Scanner};
use crate::storage::{extract_project_name, History, S3Storage, Snapshot};

pub async fn run(path: &Path, message: Option<&str>, dry_run: bool) -> Result<()> {
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
