pub mod history;
pub mod local;

pub use history::{History, Snapshot};
pub use local::LocalStorage;

use std::path::Path;

pub fn extract_project_name(path: &Path) -> String {
    path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unnamed_project")
        .to_string()
}
