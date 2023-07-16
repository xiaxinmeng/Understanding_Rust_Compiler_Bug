Rust
#[no_mangle]
unsafe extern "C" fn calculate_crc(data: *const u8, len: usize) -> u32 {
    ...
}
