rust
pub trait Ord {
    fn clamp(self, min: Self, max: Self) -> Self { .. }
}

impl f32 {
    pub fn clamp(self, min: f32, max: f32) -> f32;
}

impl f64 {
    pub fn clamp(self, min: f64, max: f64) -> f64;
}
