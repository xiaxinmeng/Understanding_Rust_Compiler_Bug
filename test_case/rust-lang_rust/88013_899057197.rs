rust
#![deny(clippy::pedantic)]

pub trait Foo {}

pub fn bar<T: Foo>() where T: Foo {}
