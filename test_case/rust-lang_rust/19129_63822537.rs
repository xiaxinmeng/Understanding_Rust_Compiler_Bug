 rust
#![feature(associated_types)]
#![crate_type="lib"]

pub trait Foo<A> {
    type B;

    fn foo(&self) -> bool { false }
}

