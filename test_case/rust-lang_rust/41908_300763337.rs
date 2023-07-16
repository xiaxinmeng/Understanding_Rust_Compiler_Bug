Rust
#[inline(never)]
pub fn myhypot(x: f64, y: f64) -> f64 {
  (x.powi(2) + y.powi(2)).sqrt()
}
