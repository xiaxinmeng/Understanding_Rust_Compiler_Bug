rust
unsafe {
	let x: &! = foo();
	let n: u64 = match Some(x) {
		None => 42,
	};
	// now `n` contains UB garbage
}
