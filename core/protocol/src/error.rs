//! Error types for protocol operations

use thiserror::Error;

pub type Result<T> = std::result::Result<T, ProtocolError>;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("CBOR encoding failed: {0}")]
    EncodingError(String),

    #[error("CBOR decoding failed: {0}")]
    DecodingError(String),

    #[error("Invalid packet: {0}")]
    InvalidPacket(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_cbor::error::Error),
}
