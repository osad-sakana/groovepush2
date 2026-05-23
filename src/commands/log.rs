use anyhow::Result;
use std::fs;

use crate::storage::{extract_project_name, LocalStorage};
use crate::utils::format_size;

pub fn run(limit: usize) -> Result<()> {
    let path = fs::canonicalize(".")?;
    let project_name = extract_project_name(&path);

    let storage = LocalStorage::new(&path)?;
    let history = storage.get_history(&project_name)?;

    match history {
        None => {
            println!("まだコミットがありません");
            println!("'gp commit' でコミットしてください");
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
