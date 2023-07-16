rust
pub fn clamp_old(this: f32, min: f32, max: f32) -> f32 {
    assert!(min <= max);
    let mut x = this;
    if x < min {
        x = min;
    }
    if x > max {
        x = max;
    }
    x
}

pub fn clamp_new(this: f32, min: f32, max: f32) -> f32 {
    assert!(min <= max);
    if this <= min {
        return min;
    }
    if this > max {
        return max;
    }
    this
}
