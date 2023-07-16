rust
pub fn lerp(t: f32, v0: f32, v1: f32) -> f32 {
    // exact, monotonic, finite, determinate, and (for v0=v1=0) consistent:
    if (v0 <= 0.0 && v1 >= 0.0) || (v0 >= 0.0 && v1 <= 0.0) {
        return t * v1 + (1.0-t) * v0;
    }
    // exact
    if t == 1.0 {
        return v1;
    }
    // exact at t=0, monotonic except near t=1,
    // bounded, determinate, and consistent:
    let x = v0 + t * (v1 - v0);
    // monotonic near t=1
    if t > 1.0 && v1 > v0 {
        f32::max(v1, x)
    } else {
        f32::min(v1, x)
    }
}
