rust
fn test(s: &i32, v: *const u32) -> &u32 {
    unsafe { &* v }
}
