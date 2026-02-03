use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "gp")]
#[command(about = "GroovePush - 音楽制作者向けS3バックアップツール")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// プロジェクトをS3にプッシュ
    Push {
        /// コミットメッセージ
        #[arg(short, long)]
        message: Option<String>,

        /// ドライラン（実際にはアップロードしない）
        #[arg(long)]
        dry_run: bool,
    },

    /// S3上のスナップショット履歴を表示
    Log {
        /// プロジェクト名
        project: Option<String>,

        /// 表示する件数
        #[arg(short = 'n', long, default_value = "10")]
        limit: usize,
    },

    /// 指定した時点の状態に復元
    Checkout {
        /// スナップショットのタイムスタンプまたはID
        snapshot: String,

        /// 復元先のディレクトリ
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// プロジェクトの初期化
    Init,

    /// 現在の状態を表示
    Status,
}
