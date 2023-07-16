
// lib.rs
#[cfg(not(target_os = "linux"))]
compile_error!("This crate only runs on Linux")

#[cfg(target_os = "linux")]
mod reallib;
#[cfg(target_os = "linux")]
pub use reallib::*;
