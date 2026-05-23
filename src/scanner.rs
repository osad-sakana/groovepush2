use crate::error::{GpError, Result};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use ignore::WalkBuilder;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

const GP_IGNORE_FILE: &str = ".gp-ignore";
const GP_DIR: &str = ".gp";

const DEFAULT_IGNORES: &[&str] = &[
    "*.tmp",
    "Backup/",
    "*.asd",
    "*.autosave",
    "*.flpbackup",
    ".DS_Store",
    "Thumbs.db",
    "*.bak",
    "*.swp",
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
    default_matcher: Gitignore,
}

impl Scanner {
    pub fn new(root: impl AsRef<Path>) -> Result<Self> {
        let root = root.as_ref().to_path_buf();
        if !root.exists() {
            return Err(GpError::DirectoryNotFound(root.display().to_string()));
        }
        let default_matcher = Self::build_default_matcher(&root);
        Ok(Self { root, default_matcher })
    }

    fn build_default_matcher(root: &Path) -> Gitignore {
        let mut builder = GitignoreBuilder::new(root);
        for &pattern in DEFAULT_IGNORES {
            let _ = builder.add_line(None, pattern);
        }
        builder.build().unwrap_or(Gitignore::empty())
    }

    pub fn scan(&self) -> Result<Vec<ScannedFile>> {
        let mut builder = WalkBuilder::new(&self.root);
        builder
            .hidden(false)
            .git_ignore(false)
            .git_global(false)
            .git_exclude(false);

        let ignore_file = self.root.join(GP_IGNORE_FILE);
        if ignore_file.exists() {
            builder.add_ignore(&ignore_file);
        }

        let gp_dir = self.root.join(GP_DIR);
        let mut files = Vec::new();

        for entry in builder.build() {
            let entry = entry.map_err(|e| GpError::IoError(std::io::Error::other(e.to_string())))?;
            let path = entry.path();

            if path.starts_with(&gp_dir) {
                continue;
            }

            if !path.is_file() {
                continue;
            }

            let relative_path = path
                .strip_prefix(&self.root)
                .map_err(|e| GpError::IoError(std::io::Error::other(e.to_string())))?
                .to_path_buf();

            if self.default_matcher.matched(&relative_path, false).is_ignore() {
                continue;
            }

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
        Ok(format!("{:x}", hasher.finalize()))
    }
}

pub fn diff_files(local: &[ScannedFile], state: &HashMap<String, String>) -> Vec<ScannedFile> {
    local
        .iter()
        .filter(|file| {
            let key = file.relative_path.to_string_lossy().to_string();
            state.get(&key).map_or(true, |h| h != &file.hash)
        })
        .cloned()
        .collect()
}
