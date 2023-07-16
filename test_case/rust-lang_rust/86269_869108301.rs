rust
pub fn lerp(t: f32, v0: f32, v1: f32) -> f32 {
    t.mul_add(v1 - v0, v0)
}
