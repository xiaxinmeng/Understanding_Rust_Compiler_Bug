rust
#[cfg(any(rustdoc, windows))]
#[doc(cfg(windows))]
pub fn my_handle() -> winapi::shared::ntdef::HANDLE { ... }
