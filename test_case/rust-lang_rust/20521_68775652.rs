 rust
#![feature(associated_types)]

pub trait Foo {
    type L;
    type R;
}

pub struct Bar<C, D>(C, D);

impl<C: Foo, D: Foo<L = C::L>> Foo for Bar<C, D> {
    type L = C::L;
    type R = C::R;
}
