use anyhow::Result;

pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

pub fn validate_project_name(name: &str) -> Result<()> {
    if name.is_empty() {
        anyhow::bail!("プロジェクト名は空にできません");
    }
    if name.contains("..") || name.contains('/') || name.contains('\\') {
        anyhow::bail!("無効なプロジェクト名: '{}'", name);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size_bytes() {
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(512), "512 B");
    }

    #[test]
    fn test_format_size_kb() {
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1536), "1.50 KB");
    }

    #[test]
    fn test_format_size_mb() {
        assert_eq!(format_size(1024 * 1024), "1.00 MB");
    }

    #[test]
    fn test_format_size_gb() {
        assert_eq!(format_size(1024 * 1024 * 1024), "1.00 GB");
    }

    #[test]
    fn test_validate_project_name_ok() {
        assert!(validate_project_name("my-project").is_ok());
        assert!(validate_project_name("project_123").is_ok());
    }

    #[test]
    fn test_validate_project_name_empty() {
        assert!(validate_project_name("").is_err());
    }

    #[test]
    fn test_validate_project_name_traversal() {
        assert!(validate_project_name("../etc").is_err());
        assert!(validate_project_name("foo/bar").is_err());
        assert!(validate_project_name("foo\\bar").is_err());
    }
}
