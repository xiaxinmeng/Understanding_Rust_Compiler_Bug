rust
fn main() {
	let mut a = Vec::<bool>::new();

	|| {
		|| {
			a.push(true)
		}
	};
}
