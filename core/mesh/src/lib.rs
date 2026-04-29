//? LinkLocal Mesh Transport crate (Phase 1 - Production)
//?
//? Platform-native BLE mesh implementation with:
//? - Linux: BlueZ via D-Bus
//? - macOS: CoreBluetooth framework
//? - Windows: Windows.Devices.Bluetooth APIs

pub mod adapter;
pub mod error;
pub mod manager;
pub mod mock;
pub mod types;
pub mod adapters;


pub use error::{ MeshError, Result };
pub use adapter::{MeshAdapter};
pub use types::{NodeId, RawPacket};
pub use manager::MeshManager;
pub use mock::MockAdapter;
pub use adapters::create_native_adapter;

/// Convenience function to create platform-native mesh manager.
pub async fn create_manager() -> Result<MeshManager> {
    let adapter = std::sync::Arc::new(create_native_adapter());
    let manager = MeshManager::new(adapter);
    manager.start().await?;
    Ok(manager)
}