rust
pub unsafe fn f(p: *const [u8; 128 * 1024]) {
    p.read_volatile();
}
