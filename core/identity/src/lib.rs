//! LinkLocal Identity Layer
//!
//! Manages cryptographic identities for nodes in the mesh.
//! Uses Ed25519 for signing and verification.
//! Enables trust and authentication in a decentralized network.

use ed25519_dalek::{Keypair, PublicKey, SigningKey, VerifyingKey};
use rand::Rng;
use serde::{Deserialize, Serialize};
use hex;

pub mod error;
pub mod identity;

pub use error::{IdentityError, Result};
pub use identity::Identity;

/// Generate a new Ed25519 keypair
pub fn generate_keypair() -> Identity {
    let mut rng = rand::thread_rng();
    let signing_key = SigningKey::generate(&mut rng);
    let keypair = Keypair::from(signing_key);

    Identity::new(keypair)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let identity = generate_keypair();
        assert!(!identity.public_key_hex().is_empty());
    }

    #[test]
    fn test_sign_and_verify() {
        let identity = generate_keypair();
        let message = b"test message";

        let signature = identity.sign(message);
        assert!(identity.verify(message, &signature));
    }

    #[test]
    fn test_invalid_signature() {
        let identity = generate_keypair();
        let message = b"test message";
        let wrong_message = b"different message";

        let signature = identity.sign(message);
        assert!(!identity.verify(wrong_message, &signature));
    }
}
