rust
pub fn from_be_array_intrinsic(bytes: [u8; 4]) -> u32 {
    u32::from_be_bytes(bytes)
}
