rust
fn clamp(mut self, min: f32, max: f32) -> f32 {
    assert!(min <= max);
    if self <= min {
        self = min;
    } else if self > max {
        self = max;
    }
    self
}
