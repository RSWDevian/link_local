//! Desktop app state management

use serde::{Deserialize, Serialize};

/// Serializable application state for persistence
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub node_id: String,
    pub node_name: String,
    pub auto_sync: bool,
    pub mesh_enabled: bool,
}

impl AppConfig {
    pub fn new(node_id: String, node_name: String) -> Self {
        Self {
            node_id,
            node_name,
            auto_sync: true,
            mesh_enabled: true,
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::new("node-default".to_string(), "LinkLocal Node".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = AppConfig::new("test-node".into(), "Test Node".into());
        assert_eq!(config.node_id, "test-node");
        assert!(config.auto_sync);
        assert!(config.mesh_enabled);
    }
}
