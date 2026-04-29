//? LinkLocal Utilities
//?
//? Common utilities and helpers used across LinkLocal crates

pub mod logger;

pub use logger::init_logger;

/// Initialize the logger configuration
/// Call this early in the application startup
pub fn setup() {
    init_logger();
}
