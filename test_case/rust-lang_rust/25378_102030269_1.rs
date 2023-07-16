 rust
pub fn wrapping_neg(self) -> $T {
    self.overflowing_neg().0
}
