rust
let x: Option<&!> = foo();
match x {
	Some(x) => {
		// This branch might be accessible if we create the reference unsafely.
	}
	None => {}
}
