
fn as_secs_f64(&self) -> f64 {
    self.as_secs() as f64 + self.subsec_nanos() as f64 * 1e-9
}
