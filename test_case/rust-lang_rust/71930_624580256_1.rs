rust
fn g() -> isize {
	let x = f();
    match x {
        true => 1,
        false => 0,
    }
}
