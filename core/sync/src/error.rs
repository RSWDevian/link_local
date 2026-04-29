//! Error types for sync operations

use thiserror::Error;

pub type Result<T> = std::result::Result<T, SyncError>;

#[derive(Error, Debug)]
pub enum SyncError {
    #[error("Decode error: {0}")]
    DecodeError(String),

    #[error("Encode error: {0}")]
    EncodeError(String),

    #[error("Apply update failed: {0}")]
    ApplyUpdateFailed(String),

    #[error("Document error: {0}")]
    DocumentError(String),
}
