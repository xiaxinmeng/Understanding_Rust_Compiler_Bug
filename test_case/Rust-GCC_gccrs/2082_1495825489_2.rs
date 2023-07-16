rust
match f { // f: (i32, i32, i32)
	(1, a, _) | (2, _, a) => {
		// do something with a here
	}
	_ => {}
}
