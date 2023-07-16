rust
extern crate a;

struct Bar(a::Foo);

impl Bar {
	fn zero(&self) -> &a::Foo {
		&self.0
	}
}

fn main() {
}
