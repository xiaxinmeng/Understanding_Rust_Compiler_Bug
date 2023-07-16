rust
#![feature(return_position_impl_trait_in_trait)]

trait Foo {
    fn bar() -> impl std::fmt::Display;
}

impl Foo for () {
    fn bar() -> () {}
}
