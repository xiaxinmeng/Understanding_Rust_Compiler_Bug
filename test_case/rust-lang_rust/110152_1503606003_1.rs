rust
#[cfg(target_pointer_width = "32")]
pub type SOCKET = u32;
#[cfg(target_pointer_width = "64")]
pub type SOCKET = u64;
