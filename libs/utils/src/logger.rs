//? Logger Conofiguration

use log::LevelFilter;

/// Initialize logging with sane defaults
pub fn init_logger() {
    let _ = env_logger::builder()
        .filter_level(LevelFilter::Info)
        .try_init();
}

/// Initialize logging with a specific level (for testing/debugging)
pub fn init_logger_with_level(level: LevelFilter) {
    let _ = env_logger::builder()
        .filter_level(level)
        .try_init();
}
