rust
// main crate
trait FooBar {
    fn dostuff(self);
}

#[derive(...)]
struct Struct<T>(T);

// derive macro expanded code
impl<T> FooBar for Struct<T> {
    fn dostuff(self) {
		use std::collections::BinaryHeap;
		let mut bh = BinaryHeap::new();
		bh.push(self.0);
    }
}
