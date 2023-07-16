rust
pub fn mod_euc(self, rhs: Self) -> Self {
	let r = self % rhs;
	let r_pos = [r + rhs, r - rhs][(rhs < 0) as usize];
	[r, r_pos][(r < 0) as usize)]
}
