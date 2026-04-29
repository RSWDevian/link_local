//! Identity structure and operations

use ed25519_dalek::{Keypair, Signature, SigningKey, VerifyingKey};
use hex;

/// An identity represented by an Ed25519 keypair
/// This is the cryptographic identity of a node in the mesh
#[derive(Clone)]
pub struct Identity {
    keypair: Keypair,
}

impl Identity {
    pub fn new(keypair: Keypair) -> Self {
        Self { keypair }
    }

    /// Get the public key as a hex string
    pub fn public_key_hex(&self) -> String {
        hex::encode(self.keypair.public.as_bytes())
    }

    /// Get the public key
    pub fn public_key(&self) -> [u8; 32] {
        *self.keypair.public.as_bytes()
    }

    /// Sign a message with this identity
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        let sig = self.keypair.sign(message);
        sig.to_bytes().to_vec()
    }

    /// Verify a signature against this identity's public key
    pub fn verify(&self, message: &[u8], signature: &[u8]) -> bool {
        if signature.len() != 64 {
            return false;
        }

        let sig = match Signature::from_bytes(&[0u8; 64][..].iter()
            .zip(signature.iter())
            .map(|(a, b)| b)
            .copied()
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()) {
            Ok(s) => s,
            Err(_) => return false,
        };

        self.keypair.public.verify_strict(message, &sig).is_ok()
    }

    /// Get a short representation of this identity (first 8 chars of public key hex)
    pub fn short_id(&self) -> String {
        let hex = self.public_key_hex();
        hex.chars().take(8).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use ed25519_dalek::SigningKey;

    fn create_test_identity() -> Identity {
        let mut rng = rand::thread_rng();
        let signing_key = SigningKey::generate(&mut rng);
        let keypair = Keypair::from(signing_key);
        Identity::new(keypair)
    }

    #[test]
    fn test_public_key_format() {
        let identity = create_test_identity();
        let hex = identity.public_key_hex();
        assert_eq!(hex.len(), 64); // 32 bytes = 64 hex chars
    }

    #[test]
    fn test_short_id() {
        let identity = create_test_identity();
        let short = identity.short_id();
        assert_eq!(short.len(), 8);
    }
}
