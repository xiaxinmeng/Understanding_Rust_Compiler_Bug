rust
pub extern "Rust" fn ret_u64_u128() -> (u64, u128) {
    (1, 0x20000000000000003)
}
