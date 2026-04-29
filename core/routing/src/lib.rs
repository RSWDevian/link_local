//! LinkLocal Routing Layer
//!
//! Handles packet deduplication, TTL management, and forwarding decisions.
//! Keeps track of seen packets to prevent infinite loops in the mesh.

use linklocal_protocol::Packet;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub mod dedup;
pub mod error;

pub use dedup::DedupCache;
pub use error::{RoutingError, Result};

/// RoutingEngine manages packet forwarding decisions
/// It maintains state about seen packets and decides whether
/// to forward based on TTL and deduplication.
pub struct RoutingEngine {
    dedup_cache: Arc<Mutex<DedupCache>>,
    max_cache_size: usize,
}

impl RoutingEngine {
    /// Create a new routing engine with a cache size
    pub fn new(max_cache_size: usize) -> Self {
        Self {
            dedup_cache: Arc::new(Mutex::new(DedupCache::new())),
            max_cache_size,
        }
    }

    /// Determine if a packet should be forwarded
    /// Returns true if packet passes dedup and TTL checks
    pub fn should_forward(&self, packet: &Packet) -> bool {
        let mut cache = self.dedup_cache.lock().expect("mutex poisoned");

        // Check if TTL is 0
        if packet.ttl == 0 {
            return false;
        }

        // Check if already seen
        if cache.contains(&packet.id) {
            return false;
        }

        // Add to cache
        cache.insert(packet.id);

        true
    }

    /// Get the current cache size
    pub fn cache_size(&self) -> usize {
        let cache = self.dedup_cache.lock().expect("mutex poisoned");
        cache.size()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_routing_decision() {
        let engine = RoutingEngine::new(1000);
        let packet = Packet::new("node-1".to_string(), vec![1, 2, 3]);

        // First packet should forward
        assert!(engine.should_forward(&packet));

        // Duplicate should not forward
        assert!(!engine.should_forward(&packet));
    }

    #[test]
    fn test_zero_ttl() {
        let engine = RoutingEngine::new(1000);
        let packet = Packet::with_ttl("node-1".to_string(), vec![1, 2, 3], 0);

        assert!(!engine.should_forward(&packet));
    }
}
