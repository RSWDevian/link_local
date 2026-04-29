//! Error types for identity operations

use thiserror::Error;

pub type Result<T> = std::result::Result<T, IdentityError>;

#[derive(Error, Debug)]
pub enum IdentityError {
    #[error("Invalid signature: {0}")]
    InvalidSignature(String),

    #[error("Key generation failed: {0}")]
    KeyGenerationFailed(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),
}
