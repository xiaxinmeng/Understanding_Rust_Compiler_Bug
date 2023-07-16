rust
#[link(name = "p", kind = "static", modifiers = "+whole-archive")]
extern "C" {
	#[no_mangle]
	pub fn add(a: u32, b: u32) -> u32;
}
