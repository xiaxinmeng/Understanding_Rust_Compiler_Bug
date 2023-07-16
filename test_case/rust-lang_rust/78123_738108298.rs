rust
fn foo(x: Option<&!>) -> u64 {
	match x {
		None => 42,
	}
}
