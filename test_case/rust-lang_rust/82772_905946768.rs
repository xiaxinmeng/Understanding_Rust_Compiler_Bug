rust
match Box::new(true) {
    Box(true, _) => {}
    box true => {} //~ ERROR unreachable pattern
	_ => {}
}

match Box::new(true) {
    box true => {}
	box false => {}
	Box(_, _) => {} //~ ERROR unreachable pattern
}
