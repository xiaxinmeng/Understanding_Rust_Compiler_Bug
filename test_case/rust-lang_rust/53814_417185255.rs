rust
use std::fmt;

pub struct Foo;

impl fmt::Display for Foo {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        panic!()
    }
}

#[test]
fn something() {
    panic!("{}", Foo);
}
