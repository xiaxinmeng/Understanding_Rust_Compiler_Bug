rust
#![warn(unused_lifetimes)]

pub struct Thingy<'x> {
	x: &'x u32,
	y: &'static str,
}

impl<'x> Thingy<'x> {
	pub fn new(x: &'x u32) -> Self {
		Self{x, y: "hello"}
	}

	pub async fn foo(&self) -> &'static str {
		self.y
	}

	pub async fn bar(&self) -> u32 {
		*self.x
	}
}
