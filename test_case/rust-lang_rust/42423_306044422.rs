rust
fn fmax(a: f32, b: f32) -> f32 {
    if b.is_nan() || b <= a { a } else { b }
}
fn fmin(a: f32, b: f32) -> f32 {
    if b.is_nan() || b >= a { a } else { b }
}
