rust
// lib.rs of crate foo

mod a {
    pub trait Foo {}
}

pub use a::Foo as Afoo;

pub fn foo<T: Afoo>() {}
