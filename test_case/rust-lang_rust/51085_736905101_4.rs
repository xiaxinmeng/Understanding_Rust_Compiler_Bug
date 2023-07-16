rust
match x { // x: Result<T, !>
	Ok(_) => {}
}
let Ok(y) = x; // x: Result<T, !>
match x {} // x: !
match x {} // x: (u64, !, String)
match x {} // x: Result<!, !>
match x { // x: &!
	_ => {} // necessary because `&!` is considered inhabited
}
match x { // x: &!
	&_ => {} // this one is unreachable, because now the `_` has type `!`.
			 // this is a bit awkward: the branch is unreachable, but if we remove it we get
			 // a non-exhaustive match.
}
match x { // x: Result<T, &!>
	Ok(_) => {}
	Err(_) => {} // necessary because `&!` is considered inhabited
}
fn safe_unwrap<T>(x: &Result<T, !>) -> &T {
    match x {
        Ok(x) => x,
		// other branch not needed because the `Err` case contains an actual `!`, not a `&!`
    }
}
fn safe_unwrap<T>(x: &Result<T, !>) -> &T {
    match x.as_ref() {
        Ok(x) => x,
		Err(_) => {} // necessary because `&!` is considered inhabited
    }
}
