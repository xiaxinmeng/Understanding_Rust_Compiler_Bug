rust
pub mod foo {
    #![doc(cfg(feature = "foo"))]
    pub struct Foo;
}

#[doc(cfg(feature = "foo"))]
pub use foo::Foo;
