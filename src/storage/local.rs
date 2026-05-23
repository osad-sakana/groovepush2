use crate::error::{GpError, Result};
use crate::scanner::ScannedFile;
use crate::storage::history::History;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub struct LocalStorage {
    gp_dir: PathBuf,
}

impl LocalStorage {
    pub fn new(project_root: &Path) -> Result<Self> {
        let gp_dir = project_root.join(".gp");
        if !gp_dir.exists() {
            return Err(GpError::NotInitialized);
        }
        Ok(Self { gp_dir })
    }

    pub fn get_current_state(&self) -> Result<HashMap<String, String>> {
        let path = self.gp_dir.join("current_state.json");
        if !path.exists() {
            return Ok(HashMap::new());
        }
        let content = fs::read_to_string(&path)?;
        serde_json::from_str(&content).map_err(|e| GpError::StorageError(e.to_string()))
    }

    pub fn save_state(&self, files: &[ScannedFile]) -> Result<()> {
        let state: HashMap<String, String> = files
            .iter()
            .map(|f| (f.relative_path.to_string_lossy().to_string(), f.hash.clone()))
            .collect();

        let path = self.gp_dir.join("current_state.json");
        let body = serde_json::to_string_pretty(&state)
            .map_err(|e| GpError::StorageError(e.to_string()))?;
        fs::write(&path, body)?;
        Ok(())
    }

    pub fn get_history(&self, project_name: &str) -> Result<Option<History>> {
        let path = self.gp_dir.join("history.json");
        if !path.exists() {
            return Ok(None);
        }
        let content = fs::read_to_string(&path)?;
        let history: History = serde_json::from_str(&content)
            .map_err(|e| GpError::StorageError(e.to_string()))?;
        if history.project_name != project_name {
            return Ok(None);
        }
        Ok(Some(history))
    }

    pub fn save_history(&self, history: &History) -> Result<()> {
        let path = self.gp_dir.join("history.json");
        let body = serde_json::to_string_pretty(history)
            .map_err(|e| GpError::StorageError(e.to_string()))?;
        fs::write(&path, body)?;
        Ok(())
    }

    pub fn save_blobs(&self, files: &[ScannedFile]) -> Result<usize> {
        if files.is_empty() {
            return Ok(0);
        }

        let blobs_dir = self.gp_dir.join("blobs");
        fs::create_dir_all(&blobs_dir)?;

        let pb = ProgressBar::new(files.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} blobs")
                .expect("プログレスバーのテンプレートエラー")
                .progress_chars("#>-"),
        );

        let mut saved_count = 0;
        for file in files {
            let dest = blobs_dir.join(&file.hash);
            if !dest.exists() {
                fs::copy(&file.absolute_path, &dest)?;
                saved_count += 1;
            }
            pb.inc(1);
        }

        pb.finish_and_clear();
        Ok(saved_count)
    }

    pub fn read_blob(&self, hash: &str) -> Result<Vec<u8>> {
        let path = self.gp_dir.join("blobs").join(hash);
        fs::read(&path).map_err(|_| GpError::BlobNotFound(hash.to_string()))
    }
}
