//! LinkLocal Protocol Layer
//! 
//! Defines packet structures, serialization (CBOR), and encoding/decoding logic.
//! This is the foundational communication layer for the BLE mesh network.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod packet;
pub mod error;
pub mod codec;

pub use packet::Packet;
pub use error::{ProtocolError, Result};
pub use codec::{encode_packet, decode_packet};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_creation() {
        let packet = Packet::new(
            "node-1".to_string(),
            vec![1, 2, 3, 4],
        );
        assert_eq!(packet.source, "node-1");
        assert_eq!(packet.ttl, 32);
        assert_eq!(packet.payload, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_cbor_roundtrip() {
        let packet = Packet::new(
            "node-1".to_string(),
            vec![1, 2, 3],
        );
        let encoded = encode_packet(&packet).expect("encode failed");
        let decoded = decode_packet(&encoded).expect("decode failed");
        assert_eq!(decoded.source, packet.source);
        assert_eq!(decoded.payload, packet.payload);
    }
}
