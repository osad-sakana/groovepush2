use crate::error::{GpError, Result};
use ignore::WalkBuilder;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

const GP_IGNORE_FILE: &str = ".gp-ignore";
const GP_DIR: &str = ".gp";

/// DAWの一時ファイルなど、デフォルトで除外するパターン
const DEFAULT_IGNORES: &[&str] = &[
    // Ableton Live
    "*.tmp",
    "Backup/",
    "*.asd",
    // Logic Pro
    "*.autosave",
    // FL Studio
    "*.flpbackup",
    // 一般的な一時ファイル
    ".DS_Store",
    "Thumbs.db",
    "*.bak",
    "*.swp",
    // GroovePush管理フォルダ
    ".gp/",
];

#[derive(Debug, Clone)]
pub struct ScannedFile {
    pub relative_path: PathBuf,
    pub absolute_path: PathBuf,
    pub size: u64,
    pub hash: String,
}

pub struct Scanner {
    root: PathBuf,
    ignore_patterns: Vec<String>,
}

impl Scanner {
    pub fn new(root: impl AsRef<Path>) -> Result<Self> {
        let root = root.as_ref().to_path_buf();

        if !root.exists() {
            return Err(GpError::DirectoryNotFound(root.display().to_string()));
        }

        let ignore_patterns = Self::load_ignore_patterns(&root);

        Ok(Self {
            root,
            ignore_patterns,
        })
    }

    fn load_ignore_patterns(root: &Path) -> Vec<String> {
        let mut patterns: Vec<String> = DEFAULT_IGNORES.iter().map(|s| s.to_string()).collect();

        let ignore_file = root.join(GP_IGNORE_FILE);
        if ignore_file.exists() {
            if let Ok(content) = fs::read_to_string(&ignore_file) {
                for line in content.lines() {
                    let line = line.trim();
                    if !line.is_empty() && !line.starts_with('#') {
                        patterns.push(line.to_string());
                    }
                }
            }
        }

        patterns
    }

    pub fn scan(&self) -> Result<Vec<ScannedFile>> {
        let mut builder = WalkBuilder::new(&self.root);

        builder
            .hidden(false)
            .git_ignore(false)
            .git_global(false)
            .git_exclude(false);

        for pattern in &self.ignore_patterns {
            builder.add_ignore(pattern);
        }

        let ignore_file = self.root.join(GP_IGNORE_FILE);
        if ignore_file.exists() {
            builder.add_ignore(&ignore_file);
        }

        let mut files = Vec::new();
        let gp_dir = self.root.join(GP_DIR);

        for entry in builder.build() {
            let entry = entry.map_err(|e| GpError::IoError(std::io::Error::other(e.to_string())))?;
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            if path.starts_with(&gp_dir) {
                continue;
            }

            let relative_path = path
                .strip_prefix(&self.root)
                .map_err(|e| GpError::IoError(std::io::Error::other(e.to_string())))?
                .to_path_buf();

            let metadata = fs::metadata(path)?;
            let hash = self.compute_hash(path)?;

            files.push(ScannedFile {
                relative_path,
                absolute_path: path.to_path_buf(),
                size: metadata.len(),
                hash,
            });
        }

        Ok(files)
    }

    fn compute_hash(&self, path: &Path) -> Result<String> {
        let content = fs::read(path)?;
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }

    pub fn root(&self) -> &Path {
        &self.root
    }
}

/// 2つのファイルリストを比較し、変更されたファイルを検出
pub fn diff_files(
    local: &[ScannedFile],
    remote: &HashMap<String, String>,
) -> Vec<ScannedFile> {
    local
        .iter()
        .filter(|file| {
            let key = file.relative_path.to_string_lossy().to_string();
            match remote.get(&key) {
                Some(remote_hash) => remote_hash != &file.hash,
                None => true,
            }
        })
        .cloned()
        .collect()
}
