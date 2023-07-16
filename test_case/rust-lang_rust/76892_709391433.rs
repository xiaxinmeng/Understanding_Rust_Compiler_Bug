rust
pub unsafe fn volatile_set_memory(slice: &mut [u8; 32], n: u8) {
    let ptr = slice as *mut [u8; 32];
    ptr.write_volatile([n; 32]);
}
