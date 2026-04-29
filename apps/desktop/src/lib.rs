//! LinkLocal Desktop Application
//!
//! A Tauri + React application for the LinkLocal mesh communication system.
//! This module provides the backend Rust logic that communicates with the UI.

use linklocal_protocol::Packet;
use linklocal_mesh::MeshNetwork;
use linklocal_routing::RoutingEngine;
use linklocal_identity::Identity;
use std::sync::Arc;

pub mod state;

/// Main application state manager
pub struct AppState {
    routing_engine: RoutingEngine,
    node_identity: Option<Identity>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            routing_engine: RoutingEngine::new(10000),
            node_identity: None,
        }
    }

    pub fn set_identity(&mut self, identity: Identity) {
        self.node_identity = Some(identity);
    }

    pub fn get_identity(&self) -> Option<&Identity> {
        self.node_identity.as_ref()
    }

    pub fn routing_engine(&self) -> &RoutingEngine {
        &self.routing_engine
    }

    pub fn should_forward_packet(&self, packet: &Packet) -> bool {
        self.routing_engine.should_forward(packet)
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_creation() {
        let state = AppState::new();
        assert!(state.get_identity().is_none());
    }

    #[test]
    fn test_set_identity() {
        let mut state = AppState::new();
        let identity = linklocal_identity::generate_keypair();
        state.set_identity(identity.clone());
        
        assert!(state.get_identity().is_some());
    }
}
