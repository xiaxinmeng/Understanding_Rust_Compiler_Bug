rust
pub fn rem_euclid(self, rhs: f64) -> f64 {
    self - self.div_euclid(rhs) * rhs
}
