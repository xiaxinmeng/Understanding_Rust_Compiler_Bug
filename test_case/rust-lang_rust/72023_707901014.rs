rust
pub fn f(a: i64, b: i64) -> u64 {
    if a < b {
        0
    } else {
        a.wrapping_sub(b) as u64
    }
}
