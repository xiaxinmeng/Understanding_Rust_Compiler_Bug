rust
pub fn lerp(t: f32, v0: f32, v1: f32) -> f32 {
    let naive = ((1.0 - t) * v0) + (t * v1);
    if v0 < v1 {
        naive.clamp(v0, v1)
    } else {
        naive.clamp(v1, v0)
    }
}

pub fn lerp_known(t: f32) -> f32 {
    lerp(t, 7.0, 13.0)
}
