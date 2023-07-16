Rust
#![feature(const_generics)]

pub trait Foo {}

impl<T, const N: usize> Foo for [T; N]
where
    Self:FooImpl<{N==0}>
{}

trait FooImpl<const IS_ZERO:bool>{}
