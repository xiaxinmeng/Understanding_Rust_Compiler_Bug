rust
extern "C" {
	fn a(_: i32, ...);
}

fn main() {
	unsafe { a(123, 0u8) };
}
