Rust
#![feature(optin_builtin_traits)]

pub struct Foo;
pub auto trait Xyz {}
impl !Xyz for Foo {}
impl !Xyz for Box<Foo> {}

impl<T: Xyz> From<T> for Box<Foo> {
    fn from(_: T) -> Self { Box::new(Foo) }
}

fn main() {
    let _foo = <Box<Foo>>::from(());
}
