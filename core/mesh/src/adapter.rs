//! Mesh adapter trait for pluggable transport implementations

use async_trait::async_trait;

/// MeshAdapter defines the interface for any mesh transport layer.
/// Implementations could be: BLE, WiFi Direct, LoRa, TCP mock, etc.
///
/// This trait enables testing with mock adapters and future support
/// for multiple simultaneous transports.
#[async_trait]
pub trait MeshAdapter: Send + Sync {
    /// Initialize and start the adapter
    async fn start(&self);

    /// Broadcast data to all neighbors
    /// In BLE, this would be a characteristic write/advertise
    async fn broadcast(&self, data: Vec<u8>);

    /// Receive data from neighbors (blocking)
    /// Returns Some(data) if received, None if timeout/error
    async fn receive(&self) -> Option<Vec<u8>>;

    /// Stop the adapter gracefully
    async fn stop(&self) {}
}

/// Mock adapter for testing and development
pub struct MockAdapter {
    name: String,
}

impl MockAdapter {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[async_trait]
impl MeshAdapter for MockAdapter {
    async fn start(&self) {
        log::info!("Mock adapter {} started", self.name);
    }

    async fn broadcast(&self, data: Vec<u8>) {
        log::debug!("Mock adapter {} broadcast {} bytes", self.name, data.len());
    }

    async fn receive(&self) -> Option<Vec<u8>> {
        // Mock: always returns None for now
        None
    }

    async fn stop(&self) {
        log::info!("Mock adapter {} stopped", self.name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_adapter() {
        let adapter = MockAdapter::new("test".to_string());
        adapter.start().await;
        adapter.broadcast(vec![1, 2, 3]).await;
        adapter.stop().await;
    }
}
