rust
pub unsafe fn load_unaligned(p: *const i64) -> i64 {
    ptr::read_unaligned(p)
}
