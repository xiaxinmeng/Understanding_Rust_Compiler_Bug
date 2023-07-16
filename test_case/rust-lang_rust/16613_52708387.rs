
fn to_int(&self) -> Option<int> { 
    if self.is_finite() {
        Some(*self as int) 
    } else {
        None
    }
}
