rust
pub struct Xxxxx;

impl Xxxxx {
	#[inline(never)]
	pub fn pppp() {
		// The debuginfo for this inlined function call expects `pppp`
		// to be in the same unit as the declaration of `qqqq`.
		qqqq();
	}
}

#[inline(always)]
fn qqqq() {
	String::from("s");
}
