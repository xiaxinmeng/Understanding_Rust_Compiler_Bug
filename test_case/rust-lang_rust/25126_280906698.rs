 rust
use std::default::Default;

struct Foo;

impl<'a> Default for &'a Foo {
    fn default() -> &'a Foo {
        panic!();
    }
}

fn main() {}
