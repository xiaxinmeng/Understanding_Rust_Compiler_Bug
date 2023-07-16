rust
fn foo(ptr: *const bool) { 
	let _: bool = { *ptr };
}
