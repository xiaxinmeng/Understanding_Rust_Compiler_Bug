rust
use std::ops::Deref;
use std::borrow::Borrow;

struct A(Box<Borrow<u32>>);

impl Deref for A {
    type Target = u32;

    fn deref(&self) -> &u32 {
        self.0.borrow()
    }
}

fn main() {}
