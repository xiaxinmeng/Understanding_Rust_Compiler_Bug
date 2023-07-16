
#![feature(associated_types)]

pub trait Foo {
    type T;
}

pub trait Bar {
    type F: Foo;
}

pub trait TyEq<A> {}
impl<A> TyEq<A> for A {}

pub fn stuff<B: Bar>(b: B) where <B::F as Foo>::T : TyEq<()> {
}

pub fn main() {
}
