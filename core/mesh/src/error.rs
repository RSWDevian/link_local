//! Error types for mesh operations

use thiserror::Error;

pub type Result<T> = std::result::Result<T, MeshError>;

#[derive(Error, Debug)]
pub enum MeshError {
    #[error("Adapter error: {0}")]
    AdapterError(String),

    #[error("Not started")]
    NotStarted,

    #[error("Receive timeout")]
    ReceiveTimeout,
}
