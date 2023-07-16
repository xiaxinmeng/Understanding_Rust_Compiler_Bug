rust
fn round(x: f32) {
    ((x.abs() + (0.25 - 0.5 * f32::EPSILON)) + (0.25 + 0.5 * f32::EPSILON)).floor().copysign(x)
}
