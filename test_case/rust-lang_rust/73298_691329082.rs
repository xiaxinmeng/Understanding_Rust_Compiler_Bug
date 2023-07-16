rust
#![feature(const_generics)]
use std::convert::AsMut;
use std::default::Default;

trait Foo: Sized {
    type Baz: Default + AsMut<[u8]>;
    fn bar() {
        Self::Baz::default().as_mut();
    }
}

impl Foo for () {
    type Baz = [u8; 1 * 1];
    //type Baz = [u8; 1];
}

fn main() {
    <() as Foo>::bar();
}
