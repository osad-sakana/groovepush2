use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn run(path: &Path) -> Result<()> {
    let gp_dir = path.join(".gp");
    if !gp_dir.exists() {
        fs::create_dir_all(&gp_dir)?;
    }

    let ignore_file = path.join(".gp-ignore");
    if !ignore_file.exists() {
        let default_content = r#"# GroovePush 除外設定
# DAWの一時ファイル等を除外

# Ableton Live
*.tmp
Backup/
*.asd

# Logic Pro
*.autosave

# FL Studio
*.flpbackup

# 一般的な一時ファイル
.DS_Store
Thumbs.db
*.bak
"#;
        fs::write(&ignore_file, default_content)?;
    }

    println!("GroovePush を初期化しました: {}", path.display());
    Ok(())
}
