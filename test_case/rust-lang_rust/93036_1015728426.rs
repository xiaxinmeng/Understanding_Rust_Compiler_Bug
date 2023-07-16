rust
pub fn cmp(a: u32, b: u32, c: u32) {
    assert!(a < b);
    assert!(b < c);
    assert!(a < c);
}
