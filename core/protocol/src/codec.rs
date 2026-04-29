//! CBOR codec for packet serialization/deserialization

use crate::packet::Packet;
use crate::error::{ProtocolError, Result};

/// Encode a packet to CBOR binary format
/// CBOR is ideal for BLE because it's:
/// - Dense (small byte footprint)
/// - Self-describing (no schema needed at decode time)
/// - Fast to encode/decode
pub fn encode_packet(packet: &Packet) -> Result<Vec<u8>> {
    serde_cbor::to_vec(packet).map_err(|e| {
        ProtocolError::EncodingError(format!("Failed to encode packet: {}", e))
    })
}

/// Decode a packet from CBOR binary format
pub fn decode_packet(data: &[u8]) -> Result<Packet> {
    if data.is_empty() {
        return Err(ProtocolError::DecodingError(
            "Empty data cannot be decoded".to_string(),
        ));
    }

    serde_cbor::from_slice(data).map_err(|e| {
        ProtocolError::DecodingError(format!("Failed to decode packet: {}", e))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_roundtrip() {
        let packet = Packet::new("test-source".into(), vec![1, 2, 3, 4, 5]);
        let encoded = encode_packet(&packet).expect("encode failed");
        let decoded = decode_packet(&encoded).expect("decode failed");

        assert_eq!(decoded.id, packet.id);
        assert_eq!(decoded.source, packet.source);
        assert_eq!(decoded.ttl, packet.ttl);
        assert_eq!(decoded.payload, packet.payload);
    }

    #[test]
    fn test_decode_empty_data() {
        let result = decode_packet(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_payload_preservation() {
        let payload = vec![0xFF, 0xAA, 0xBB, 0xCC, 0xDD];
        let packet = Packet::new("node".into(), payload.clone());
        let encoded = encode_packet(&packet).expect("encode failed");
        let decoded = decode_packet(&encoded).expect("decode failed");
        assert_eq!(decoded.payload, payload);
    }
}
