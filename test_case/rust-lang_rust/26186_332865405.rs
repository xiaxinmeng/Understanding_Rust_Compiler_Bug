rust
use std::ops::{Deref, DerefMut};

struct Foo;

impl Deref for Foo {
    type Target = FnMut() + 'static;
    fn deref(&self) -> &Self::Target {
        unimplemented!()
    }
}

impl DerefMut for Foo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unimplemented!()
    }
}

fn main() {
    let mut t = Foo;
    t();
}
