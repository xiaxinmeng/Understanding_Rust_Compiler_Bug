rust
impl Duration {
    pub fn as_secs_f64(&self) -> f64;
    pub fn as_secs_f32(&self) -> f32;
    pub fn from_secs_f64(secs: f64) -> Duration; // panics on overflow
    pub fn from_secs_f32(secs: f32) -> Duration; // panics on overflow
}
