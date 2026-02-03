use thiserror::Error;

#[derive(Error, Debug)]
pub enum GpError {
    #[error("ディレクトリが見つかりません: {0}")]
    DirectoryNotFound(String),

    #[error("S3エラー: {0}")]
    S3Error(String),

    #[error("データベースエラー: {0}")]
    DatabaseError(#[from] rusqlite::Error),

    #[error("IOエラー: {0}")]
    IoError(#[from] std::io::Error),

    #[error("設定エラー: {0}")]
    ConfigError(String),

    #[error("スナップショットが見つかりません: {0}")]
    SnapshotNotFound(String),

    #[error("履歴が見つかりません")]
    HistoryNotFound,

    #[error("blobが見つかりません: {0}")]
    BlobNotFound(String),
}

pub type Result<T> = std::result::Result<T, GpError>;
