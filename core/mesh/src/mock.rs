use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use async_trait::async_trait;
use tokio::sync::{mpsc, Mutex};

use crate::adapter::MeshAdapter;
use crate::error::{MeshError, Result};

/// In-memory adapter for local testing using Tokio mpsc channels.
pub struct MockAdapter {
    started: AtomicBool,
    tx: mpsc::Sender<Vec<u8>>,
    rx: Arc<Mutex<mpsc::Receiver<Vec<u8>>>>,
    advertised: Arc<Mutex<Vec<Vec<u8>>>>,
}

impl MockAdapter {
    pub fn new(buffer: usize) -> Self {
        let (tx, rx) = mpsc::channel(buffer);

        Self {
            started: AtomicBool::new(false),
            tx,
            rx: Arc::new(Mutex::new(rx)),
            advertised: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn is_started(&self) -> bool {
        self.started.load(Ordering::SeqCst)
    }

    pub async fn advertised_packets(&self) -> Vec<Vec<u8>> {
        self.advertised.lock().await.clone()
    }

    fn ensure_started(&self) -> Result<()> {
        if self.is_started() {
            Ok(())
        } else {
            Err(MeshError::NotStarted)
        }
    }
}

#[async_trait]
impl MeshAdapter for MockAdapter {
    async fn start(&self) -> Result<()> {
        if self.started.swap(true, Ordering::SeqCst) {
            return Err(MeshError::AlreadyStarted);
        }
        Ok(())
    }

    async fn advertise(&self, data: Vec<u8>) -> Result<()> {
        self.ensure_started()?;

        if data.is_empty() {
            return Err(MeshError::InvalidPacket("empty advertise payload"));
        }

        self.advertised.lock().await.push(data.clone());
        self.tx.send(data).await.map_err(|_| MeshError::ChannelClosed)
    }

    async fn scan(&self) -> Result<()> {
        self.ensure_started()?;
        Ok(())
    }

    async fn send(&self, data: Vec<u8>) -> Result<()> {
        self.ensure_started()?;

        if data.is_empty() {
            return Err(MeshError::InvalidPacket("empty send payload"));
        }

        self.tx.send(data).await.map_err(|_| MeshError::ChannelClosed)
    }

    async fn receive(&self) -> Result<Option<Vec<u8>>> {
        self.ensure_started()?;
        let mut rx = self.rx.lock().await;
        Ok(rx.recv().await)
    }
}

#[cfg(test)]
mod tests {
    use crate::adapter::MeshAdapter;
    use crate::mock::MockAdapter;

    #[tokio::test]
    async fn mock_round_trip_send_receive() {
        let adapter = MockAdapter::new(16);
        adapter.start().await.expect("start should succeed");

        let payload = vec![1, 2, 3, 4];
        adapter.send(payload.clone()).await.expect("send should succeed");

        let got = adapter
            .receive()
            .await
            .expect("receive should succeed")
            .expect("packet should exist");

        assert_eq!(got, payload);
    }

    #[tokio::test]
    async fn mock_advertise_tracks_payload() {
        let adapter = MockAdapter::new(16);
        adapter.start().await.expect("start should succeed");

        let payload = b"adv".to_vec();
        adapter
            .advertise(payload.clone())
            .await
            .expect("advertise should succeed");

        let advertised = adapter.advertised_packets().await;
        assert_eq!(advertised.len(), 1);
        assert_eq!(advertised[0], payload);
    }
}