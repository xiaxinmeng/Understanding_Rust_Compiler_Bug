rust
fn main() {
	let s = &[0x00; 4][..]; //Slice of any value
	const MAGIC_TEST: &[u32] = &[1, 2, 3, 4]; //Const slice to pattern match with
	match s {
		MAGIC_TEST => (),
		[0x00, 0x00, 0x00, 0x00] => (),
		_ => ()
	}
}
