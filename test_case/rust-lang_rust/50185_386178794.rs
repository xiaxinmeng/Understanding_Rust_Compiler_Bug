rust
pub fn mod_euc(self, rhs: Self) -> Self {
	let r = self % rhs;
	if r < 0 {
		if rhs < 0 {
			r - rhs
		} else {
			r + rhs
		}
	} else {
		r
	}
}
