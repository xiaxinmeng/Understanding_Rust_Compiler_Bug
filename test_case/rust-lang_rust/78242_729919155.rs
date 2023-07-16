rust
match x {
	0..=10 => {} // Oops, probably meant to write `0..10`
	10..=20 => {}
	_ => {}
}
