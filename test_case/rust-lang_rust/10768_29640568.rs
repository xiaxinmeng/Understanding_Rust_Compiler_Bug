 rust
// src/libstd/logging.rs
pub static DEBUG: u32 = 4;
pub static INFO: u32 = 3;
pub static WARN: u32 = 2;
pub static ERROR: u32 = 1;

// only one macro
macro_rules! log_enabled($lvl:expr) ( ... )
