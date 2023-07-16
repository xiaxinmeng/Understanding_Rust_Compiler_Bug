rust
fn main() {
	let _ = { if false {} 0 };	//< Allowed
	let _ = { () 0 };	//< Errors
}
