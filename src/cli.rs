use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gp")]
#[command(about = "GroovePush - 音楽制作者向けローカルバージョン管理ツール")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// スナップショットをコミット
    Commit {
        /// コミットメッセージ
        #[arg(short, long)]
        message: Option<String>,

        /// ドライラン（実際には保存しない）
        #[arg(long)]
        dry_run: bool,
    },

    /// スナップショット履歴を表示
    Log {
        /// 表示する件数
        #[arg(short = 'n', long, default_value = "10")]
        limit: usize,
    },

    /// 指定した時点の状態に復元
    Checkout {
        /// スナップショットのIDまたはプレフィックス
        snapshot: String,
    },

    /// プロジェクトの初期化
    Init,

    /// 現在の状態を表示
    Status,
}
