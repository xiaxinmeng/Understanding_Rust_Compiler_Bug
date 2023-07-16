 rust
fn update_calculation(&mut self) -> Result<(), ()> {
    let tmp1 = match self.val1.checked_add(self.val2) {
        Some(v) => v,
        None => return Err(())
    };

    let tmp2 = match tmp1.checked_add(self.val3) {
        Some(v) => v,
        None => return Err(())
    };

    self.calculation_result = tmp2;
    Ok(())
}

pub fn set_val1(&mut self, value: int) {
    let previous_value = self.val1;
    self.val1 = value;
    if let Err(_) = self.update_calculation() {
        self.val1 = previous_value;
        panic!();
    }
}
