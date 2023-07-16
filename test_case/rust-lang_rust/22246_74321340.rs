 rust
use std::ops::Deref;

pub trait ToOwned {
    type Owned: Borrow<Self>;
    fn to_owned(&self) -> Self::Owned;
}

pub trait Borrow<Borrowed> {
    fn borrow(&self) -> &Borrowed;
}

pub struct Foo<B:ToOwned> {
    owned: B::Owned
}

fn foo<B:ToOwned>(this: &Foo<B>) -> &B {
    this.owned.borrow()
}

fn main() { }
