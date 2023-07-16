rust
#[cfg(test)]
mod tests {
	use test::{Bencher, black_box};

	pub fn mod_euc_branching(slf: i64, rhs: i64) -> i64 {
		let r = slf % rhs;
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

	pub fn mod_euc_branchless(slf: i64, rhs: i64) -> i64 {
		let r = slf % rhs;
		let r_pos = [r + rhs, r - rhs][(rhs < 0) as usize];
		[r, r_pos][(r < 0) as usize]
	}

	pub fn mod_euc_branchless_u(slf: i64, rhs: i64) -> i64 {
		let r = slf % rhs;
		unsafe {
			let r_pos = *[r + rhs, r - rhs].get_unchecked((rhs < 0) as usize);
			*[r, r_pos].get_unchecked((r < 0) as usize)
		}
	}

	pub fn mod_euc_branchless_2(slf: i64, rhs: i64) -> i64 {
		let r = slf % rhs;
		let r_pos = r + rhs - 2 * rhs * (rhs < 0) as i64;
		r + (r_pos - r) * (r < 0) as i64
	}

	#[bench]
	fn bench_mod_euc_branching(b: &mut Bencher) {
		b.iter(|| {
			for a in -1000..1000 {
				for b in -1000..1000 {
					if b == 0 { continue; }
					black_box(mod_euc_branching(a, b));
				}
			}
		});
	}

	#[bench]
	fn bench_mod_euc_branchless(b: &mut Bencher) {
		b.iter(|| {
			for a in -1000..1000 {
				for b in -1000..1000 {
					if b == 0 { continue; }
					black_box(mod_euc_branchless(a, b));
				}
			}
		});
	}

	#[bench]
	fn bench_mod_euc_branchless_u(b: &mut Bencher) {
		b.iter(|| {
			for a in -1000..1000 {
				for b in -1000..1000 {
					if b == 0 { continue; }
					black_box(mod_euc_branchless_u(a, b));
				}
			}
		});
	}

	#[bench]
	fn bench_mod_euc_branchless_2(b: &mut Bencher) {
		b.iter(|| {
			for a in -1000..1000 {
				for b in -1000..1000 {
					if b == 0 { continue; }
					black_box(mod_euc_branchless_2(a, b));
				}
			}
		});
	}
}
