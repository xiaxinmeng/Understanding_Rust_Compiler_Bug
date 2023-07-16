rust
pub fn select(m: mask32x4, a: i32x4, b: i32x4) -> i32x4 {
    m.select(a, b)
}
