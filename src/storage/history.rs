use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotMeta {
    pub file_count: usize,
    pub total_size: u64,
    pub changed_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub message: Option<String>,
    pub files: HashMap<String, String>,
    pub parent_id: Option<String>,
    pub meta: SnapshotMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct History {
    pub version: u32,
    pub project_name: String,
    pub head: Option<String>,
    pub snapshots: Vec<Snapshot>,
}

impl Snapshot {
    pub fn generate_id() -> String {
        Utc::now().format("%Y%m%dT%H%M%SZ").to_string()
    }

    pub fn new(
        message: Option<String>,
        files: HashMap<String, String>,
        parent_id: Option<String>,
        total_size: u64,
        changed_count: usize,
    ) -> Self {
        Self {
            id: Self::generate_id(),
            created_at: Utc::now(),
            message,
            meta: SnapshotMeta {
                file_count: files.len(),
                total_size,
                changed_count,
            },
            files,
            parent_id,
        }
    }
}

impl History {
    pub fn new(project_name: &str) -> Self {
        Self {
            version: 1,
            project_name: project_name.to_string(),
            head: None,
            snapshots: Vec::new(),
        }
    }

    pub fn add_snapshot(&mut self, snapshot: Snapshot) {
        self.head = Some(snapshot.id.clone());
        self.snapshots.push(snapshot);
    }

    pub fn find_snapshot(&self, id: &str) -> Option<&Snapshot> {
        self.snapshots.iter().find(|s| s.id == id)
    }

    pub fn find_snapshot_by_prefix(&self, prefix: &str) -> Option<&Snapshot> {
        self.snapshots
            .iter()
            .rev()
            .find(|s| s.id.starts_with(prefix))
    }

    pub fn latest_snapshot(&self) -> Option<&Snapshot> {
        self.snapshots.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_id_generation() {
        let id = Snapshot::generate_id();
        assert_eq!(id.len(), 16);
        assert!(id.contains('T'));
        assert!(id.ends_with('Z'));
    }

    #[test]
    fn test_history_serialization() {
        let history = History::new("test_project");
        let json = serde_json::to_string(&history).unwrap();
        let restored: History = serde_json::from_str(&json).unwrap();
        assert_eq!(history.project_name, restored.project_name);
    }

    #[test]
    fn test_find_snapshot_by_prefix() {
        let mut history = History::new("test");
        let mut files = HashMap::new();
        files.insert("test.txt".to_string(), "abc123".to_string());

        let snapshot = Snapshot {
            id: "20260203T143052Z".to_string(),
            created_at: Utc::now(),
            message: Some("test".to_string()),
            files,
            parent_id: None,
            meta: SnapshotMeta {
                file_count: 1,
                total_size: 100,
                changed_count: 1,
            },
        };
        history.add_snapshot(snapshot);

        assert!(history.find_snapshot_by_prefix("20260203").is_some());
        assert!(history.find_snapshot_by_prefix("202602").is_some());
        assert!(history.find_snapshot_by_prefix("20250101").is_none());
    }
}
