 rs
#[cfg(target_arch = "arm")]
pub type c_char = u8;
#[cfg(not(target_arch = "arm"))]
pub type c_char = i8;
