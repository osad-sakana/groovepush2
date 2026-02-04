use thiserror::Error;

#[derive(Error, Debug)]
pub enum GpError {
    #[error("ディレクトリが見つかりません: {0}")]
    DirectoryNotFound(String),

    #[error("S3エラー: {0}")]
    S3Error(String),

    #[error("IOエラー: {0}")]
    IoError(#[from] std::io::Error),

    #[error("blobが見つかりません: {0}")]
    BlobNotFound(String),
}

pub type Result<T> = std::result::Result<T, GpError>;
