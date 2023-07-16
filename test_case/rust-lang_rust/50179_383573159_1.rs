rust
pub fn mod_euc(self, rhs: f64) -> f64 {
    ((self % rhs) + rhs.abs()) % rhs
}
