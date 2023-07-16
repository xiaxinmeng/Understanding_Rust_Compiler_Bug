 rust
fn update_calculation(&mut self) {
    self.calculation_result = self.val1.checked_add(self.val2).unwrap().checked_add(self.val3).unwrap();
}

pub fn set_val1(&mut self, value: int) {
    self.val1 = value;
    self.update_calculation();
}
