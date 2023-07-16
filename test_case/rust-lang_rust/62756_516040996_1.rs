rust
impl Duration {
    pub fn mul_f64(self, rhs: f64) -> Duration; // panics on overflow
    pub fn mul_f32(self, rhs: f32) -> Duration; // panics on overflow
    pub fn div_f64(self, rhs: f64) -> Duration;
    pub fn div_f32(self, rhs: f32) -> Duration;
}
