rust
#![feature(trait_alias)]

pub trait Foo {}
pub trait FooAlias = Foo;
