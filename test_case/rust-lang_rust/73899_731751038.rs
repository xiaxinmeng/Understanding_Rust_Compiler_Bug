Rust
#![feature(const_evaluatable_checked, min_const_generics)]
trait Foo {}

impl<const N: usize> Foo for [(); N]
where
    Self:FooImpl<{N==0}>
{}

trait FooImpl<const IS_ZERO: bool>{}

impl FooImpl<{0u8==0u8}> for [();0] {}

impl<const N:usize> FooImpl<{0u8!=0u8}> for [();N] {}

fn foo<T: Foo>(v: T) {}

fn main() {
    foo([]);
    foo([()]);
}
