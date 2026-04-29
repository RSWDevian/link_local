use std::sync::Arc;
use std::time::Duration;

use log::info;
use tokio::time::timeout;

use crate::adapter::MeshAdapter;
use crate::error::{MeshError, Result};
use crate::types::RawPacket;

/// MeshManager orchestrates transport-level operations.
///
/// Responsibilities:
/// - Lifecycle management (start, stop)
/// - Peer discovery
/// - Packet transmission/reception
/// - Error recovery
pub struct MeshManager {
    adapter: Arc<dyn MeshAdapter>,
}

impl MeshManager {
    pub fn new(adapter: Arc<dyn MeshAdapter>) -> Self {
        Self { adapter }
    }

    /// Start the mesh transport.
    pub async fn start(&self) -> Result<()> {
        info!("MeshManager: starting");
        self.adapter.start().await
    }

    /// Discover nearby peers (blocking scan).
    pub async fn discover_peers(&self) -> Result<()> {
        info!("MeshManager: discovering peers");
        self.adapter.scan().await
    }

    /// Advertise a payload to nearby peers.
    pub async fn advertise(&self, packet: RawPacket) -> Result<()> {
        if packet.is_empty() {
            return Err(MeshError::InvalidPacket("empty advertise payload"));
        }

        self.adapter.advertise(packet).await
    }

    /// Send a packet into the mesh.
    pub async fn send(&self, packet: RawPacket) -> Result<()> {
        if packet.is_empty() {
            return Err(MeshError::InvalidPacket("empty send payload"));
        }

        self.adapter.send(packet).await
    }

    /// Receive a packet from the mesh with optional timeout.
    pub async fn receive(&self) -> Result<Option<RawPacket>> {
        self.adapter.receive().await
    }

    /// Receive with timeout.
    pub async fn receive_timeout(&self, timeout_secs: u64) -> Result<Option<RawPacket>> {
        match timeout(
            Duration::from_secs(timeout_secs),
            self.adapter.receive(),
        )
        .await
        {
            Ok(Ok(packet)) => Ok(packet),
            Ok(Err(e)) => Err(e),
            Err(_) => Ok(None), // Timeout
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::MockAdapter;

    #[tokio::test]
    async fn manager_lifecycle() {
        let adapter = Arc::new(MockAdapter::new(64));
        let manager = MeshManager::new(adapter);

        manager.start().await.expect("start should work");
        manager.discover_peers().await.expect("scan should work");

        let payload = b"test".to_vec();
        manager.send(payload.clone()).await.expect("send should work");

        let received = manager
            .receive()
            .await
            .expect("receive should work")
            .expect("packet present");

        assert_eq!(received, payload);
    }

    #[tokio::test]
    async fn manager_receive_timeout() {
        let adapter = Arc::new(MockAdapter::new(64));
        let manager = MeshManager::new(adapter);

        manager.start().await.expect("start should work");

        let result = manager.receive_timeout(1).await;
        assert!(result.is_ok());
    }
}