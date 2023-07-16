 rust
pub fn lerp<T: Add + Mul<f32>>(p0: T, p1: T, pct: f32) -> T{
    p0 * (1.0 - pct) + p1 * pct
}
