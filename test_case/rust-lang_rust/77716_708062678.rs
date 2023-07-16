rs
#[cfg(target_os = "android")]
#![crate_type = "cdylib"]
#[cfg(target_os = "ios")]
#![crate_type = "staticlib"]
#[cfg(not(any(target_os = "android", target_os = "ios")))]
#![crate_type = "rlib"]
