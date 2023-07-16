 rust
pub fn test(a: u32, b: u32) -> Option<u32> {
    if likely(a == b) {
        None
    } else {
        Some(a + b)
    }
}
