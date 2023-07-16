rust
pub fn lerp(t: f32, v0: f32, v1: f32) -> f32 {
    // monotonic, consistent
    let naive = t.mul_add(v1 - v0, v0);
    // bounded
    if v0 <= v1 {
        naive.clamp(v0, v1)
    } else {
        naive.clamp(v1, v0)
    }
}
