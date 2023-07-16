 Rust
fn update_calculation(&mut self) {
    if self.val1 == 127 { return; };
    self.calculation_result = self.val1 + self.val2 + self.val3;
}
