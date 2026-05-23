use thiserror::Error;

#[derive(Error, Debug)]
pub enum GpError {
    #[error("ディレクトリが見つかりません: {0}")]
    DirectoryNotFound(String),

    #[error("ストレージエラー: {0}")]
    StorageError(String),

    #[error("IOエラー: {0}")]
    IoError(#[from] std::io::Error),

    #[error("blobが見つかりません: {0}")]
    BlobNotFound(String),

    #[error("未初期化: 'gp init' を先に実行してください")]
    NotInitialized,
}

pub type Result<T> = std::result::Result<T, GpError>;
