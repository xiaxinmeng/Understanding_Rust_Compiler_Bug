rust
pub fn mod_euc(self, rhs: f64) -> f64 {
    let r = self % rhs;
    if r < 0.0 {
        r + rhs.abs()
    } else {
        r
    }
}
