rust
fn main() {
	let a = Vec::<bool>::new();

	|| {
		|| {
			a.len()
		}
	};
}
