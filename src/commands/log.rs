use anyhow::Result;
use std::fs;

use crate::storage::{extract_project_name, S3Storage};
use crate::utils::format_size;

pub async fn run(project: Option<&str>, limit: usize) -> Result<()> {
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
