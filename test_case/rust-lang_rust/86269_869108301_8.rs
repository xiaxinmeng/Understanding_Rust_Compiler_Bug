rust
pub fn lerp(t: f32, v0: f32, v1: f32) -> f32 {
    let naive = ((1.0 - t) * v0) + (t * v1);
    if v0 == v1 {
        v0
    } else {
        naive
    }
}
