#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub use linux::LinuxAdapter;
#[cfg(target_os = "macos")]
pub use macos::MacosAdapter;
#[cfg(target_os = "windows")]
pub use windows::WindowsAdapter;

/// Get platform-native adapter.
pub fn create_native_adapter() -> Box<dyn crate::adapter::MeshAdapter + Send + Sync> {
    #[cfg(target_os = "linux")]
    {
        Box::new(LinuxAdapter::new())
    }

    #[cfg(target_os = "macos")]
    {
        Box::new(MacosAdapter::new())
    }

    #[cfg(target_os = "windows")]
    {
        Box::new(WindowsAdapter::new())
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        compile_error!("LinkLocal mesh only supports Linux, macOS, and Windows")
    }
}