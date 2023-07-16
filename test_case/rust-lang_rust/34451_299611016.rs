rust
trait A {
	fn b(&self);
}
impl<'c> A for (usize,[&'c [u8]]) {
	fn b(&self) {
	    0;
	}
}

fn main() {}
