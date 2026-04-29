//! Packet structure and definitions for LinkLocal mesh protocol.

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// Core packet structure for mesh communication.
/// Each packet has a unique ID to enable deduplication across the network.
/// TTL (Time-To-Live) prevents infinite looping.
/// Payload is CBOR-encoded application data.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Packet {
    /// Unique identifier for this packet instance
    pub id: Uuid,

    /// Node ID or identifier of the packet originator
    pub source: String,

    /// Time-to-live counter, decremented at each hop
    pub ttl: u8,

    /// Raw payload data (CBOR encoded at the application level)
    pub payload: Vec<u8>,

    /// Timestamp in seconds since UNIX epoch
    pub timestamp: u64,
}

impl Packet {
    /// Create a new packet with default values
    /// TTL defaults to 32 hops (reasonable for mesh)
    /// Timestamp is set to current system time
    pub fn new(source: String, payload: Vec<u8>) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time before epoch")
            .as_secs();

        Self {
            id: Uuid::new_v4(),
            source,
            ttl: 32,
            payload,
            timestamp: now,
        }
    }

    /// Create a packet with custom TTL
    pub fn with_ttl(source: String, payload: Vec<u8>, ttl: u8) -> Self {
        let mut packet = Self::new(source, payload);
        packet.ttl = ttl;
        packet
    }

    /// Decrement TTL and check if packet should be forwarded
    /// Returns false if TTL reaches 0
    pub fn decrement_ttl(&mut self) -> bool {
        if self.ttl > 0 {
            self.ttl -= 1;
            true
        } else {
            false
        }
    }

    /// Check if packet is still valid (TTL > 0)
    pub fn is_valid(&self) -> bool {
        self.ttl > 0
    }

    /// Get the size of this packet when encoded
    pub fn size(&self) -> usize {
        // Approximate: UUIDs (16) + source (len+data) + ttl (1) + timestamp (8) + payload
        16 + self.source.len() + 2 + 1 + 8 + self.payload.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_ttl_decrement() {
        let mut packet = Packet::new("test".into(), vec![]);
        assert!(packet.decrement_ttl());
        assert_eq!(packet.ttl, 31);
    }

    #[test]
    fn test_packet_ttl_zero() {
        let mut packet = Packet::with_ttl("test".into(), vec![], 1);
        assert!(packet.decrement_ttl());
        assert!(!packet.decrement_ttl());
    }

    #[test]
    fn test_packet_size() {
        let packet = Packet::new("node-1".into(), vec![1, 2, 3, 4]);
        assert!(packet.size() > 0);
    }
}
