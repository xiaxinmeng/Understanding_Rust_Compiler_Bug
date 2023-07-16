rust
#![feature(const_generics)]
#[allow(incomplete_features)]

struct Z<const N: usize>;

impl<const N: usize> Z<{N}> {

	fn fact() -> i32 {
		match N {
			0 => 1,
			_ => Z::<{
				match N {
					0 => 0,
					_ => N - 1,
				}
			}>::fact()
		}
	}

}
