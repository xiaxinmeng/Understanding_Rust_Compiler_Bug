rust
trait Foo {
    const BAR: usize;
}

struct Baz;

impl Foo for Baz {
    const BAR: usize = 3; // Change this to 4 and we have a compilation error.
}

fn main() {
    let arr: [u8; <Baz as Foo>::BAR] = [1, 2, 3];
}
