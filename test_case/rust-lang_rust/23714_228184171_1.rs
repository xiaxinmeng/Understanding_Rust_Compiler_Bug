 rust
use std::borrow::Borrow;

pub trait ToOwned {
    type Owned: Borrow<Self>;
    fn to_owned(&self) -> Self::Owned;
}

enum Foo {
    Bar,
    Baz { cow: Result<&'static Foo, <Foo as ToOwned>::Owned> }
}

impl ToOwned for Foo {
    type Owned = Box<Foo>;
    fn to_owned(&self) -> Box<Foo> { loop {} }
}
