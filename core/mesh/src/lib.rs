//! LinkLocal Mesh Layer
//!
//! Provides abstraction for mesh networking via BLE or other transports.
//! Uses an async trait pattern to support different implementations.

use async_trait::async_trait;
use std::sync::Arc;

pub mod adapter;
pub mod error;

pub use adapter::MeshAdapter;
pub use error::{MeshError, Result};

/// MeshNetwork coordinates packet broadcast and receive operations
/// across the mesh. It manages the underlying adapter and coordinates
/// async operations in a thread-safe manner.
pub struct MeshNetwork {
    adapter: Arc<dyn MeshAdapter>,
}

impl MeshNetwork {
    pub fn new(adapter: Arc<dyn MeshAdapter>) -> Self {
        Self { adapter }
    }

    pub async fn start(&self) {
        self.adapter.start().await;
    }

    pub async fn broadcast(&self, data: Vec<u8>) {
        self.adapter.broadcast(data).await;
    }

    pub async fn receive(&self) -> Option<Vec<u8>> {
        self.adapter.receive().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mesh_network_creation() {
        // Mock adapter would be used in real tests
        // This is a placeholder for structure verification
    }
}
