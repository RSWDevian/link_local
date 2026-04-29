//! Definition of the error types for mesh operations.

use thiserror::Error;
pub type Result<T> = std::result::Result<T, MeshError>;

#[derive(Error, Debug)]
pub enum MeshError {
    #[error("Adapter Error: Adapter not yet  started")]
    NotStarted,

    #[error("Adapter Error: Adapter already  started")]
    AlreadyStarted,

    #[error("Channel Error: Channel is closed")]
    ChannelClosed,

    #[error("Packet Error: Invalid packet: {0}")]
    InvalidPacket(&'static str),

    #[error("Adapter Error: Platform adapter not implemented")]
    NotImplemented(&'static str),

    #[error("Internal Error: {0}")]
    Internal(String),
}