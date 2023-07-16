rust
pub unsafe fn demo(x: *const u8, y: *const u8) -> isize {
    x.offset_from(y)
}
