 rust
use std::ops::Deref;

struct Foo(u8);

impl Deref for  Foo {
    type Target = u8;

    fn deref<'a>(&'a mut self) -> &'a u8 {
        &self.0
    }
}

fn main() {}
