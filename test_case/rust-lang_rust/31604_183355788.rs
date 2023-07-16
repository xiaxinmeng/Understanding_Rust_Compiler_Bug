 rust
use std::ops::Not;

struct Foo;

impl Not for Foo {
    type Output = bool;

    fn not(self) -> Self::Output {
        false
    }
}

fn main() {
    assert!(Foo);
}
