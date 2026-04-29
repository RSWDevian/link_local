//! LinkLocal Sync Layer
//!
//! Implements CRDT-based state synchronization using Yrs (Yjs for Rust).
//! Enables consistent, conflict-free data replication across the mesh.

use yrs::Doc;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub mod error;
pub mod state;

pub use error::{SyncError, Result};
use linklocal_protocol::Packet;

/// SyncManager handles distributed state synchronization
/// Uses Yrs as the CRDT layer for automatic conflict resolution
pub struct SyncManager {
    doc: Arc<Mutex<Doc>>,
    node_id: String,
}

impl SyncManager {
    pub fn new(node_id: String) -> Self {
        let doc = Doc::new();
        Self {
            doc: Arc::new(Mutex::new(doc)),
            node_id,
        }
    }

    /// Apply a remote update to the document
    pub fn apply_update(&self, update: &[u8]) -> Result<()> {
        let mut doc = self.doc.lock().expect("mutex poisoned");
        doc.apply_update_from_binary(update.to_vec())
            .map_err(|e| SyncError::DecodeError(e.to_string()))
    }

    /// Encode a document state
    pub fn get_state(&self) -> Vec<u8> {
        let doc = self.doc.lock().expect("mutex poisoned");
        let mut state = Vec::new();
        doc.encode_state_as_update(&Default::default(), &mut state);
        state
    }

    /// Get the current node ID
    pub fn node_id(&self) -> &str {
        &self.node_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_manager_creation() {
        let manager = SyncManager::new("node-1".to_string());
        assert_eq!(manager.node_id(), "node-1");
    }

    #[test]
    fn test_get_state() {
        let manager = SyncManager::new("node-1".to_string());
        let state = manager.get_state();
        assert!(state.len() > 0);
    }
}
