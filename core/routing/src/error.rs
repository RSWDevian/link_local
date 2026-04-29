//! Error types for routing operations

use thiserror::Error;

pub type Result<T> = std::result::Result<T, RoutingError>;

#[derive(Error, Debug)]
pub enum RoutingError {
    #[error("Forwarding failed: {0}")]
    ForwardingFailed(String),

    #[error("Duplicate packet (already seen)")]
    DuplicatePacket,

    #[error("TTL expired")]
    TTLExpired,
}
