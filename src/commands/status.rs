use anyhow::Result;
use std::fs;
use std::path::Path;

use crate::scanner::{diff_files, Scanner};
use crate::storage::{extract_project_name, LocalStorage};
use crate::utils::format_size;

pub fn run(path: &Path) -> Result<()> {
    let path = fs::canonicalize(path)?;
    let project_name = extract_project_name(&path);

    println!("プロジェクト: {}", project_name);

    let storage = LocalStorage::new(&path)?;
    let scanner = Scanner::new(&path)?;
    let local_files = scanner.scan()?;

    println!("ローカルファイル数: {}", local_files.len());

    let total_size: u64 = local_files.iter().map(|f| f.size).sum();
    println!("合計サイズ: {}", format_size(total_size));

    let current_state = storage.get_current_state()?;

    if current_state.is_empty() {
        println!("まだコミットされていません");
    } else {
        let changed_files = diff_files(&local_files, &current_state);
        println!("変更ファイル数: {}", changed_files.len());
    }

    Ok(())
}
