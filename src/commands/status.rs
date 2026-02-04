use anyhow::Result;
use std::fs;
use std::path::Path;

use crate::scanner::{diff_files, Scanner};
use crate::storage::{extract_project_name, S3Storage};
use crate::utils::format_size;

pub async fn run(path: &Path) -> Result<()> {
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
