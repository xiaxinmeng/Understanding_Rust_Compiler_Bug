 rust
pub fn abs(self) -> $T {
    if self.is_negative() {
        self.wrapping_neg()
    } else {
        self
    }
}
