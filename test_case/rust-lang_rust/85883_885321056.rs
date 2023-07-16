rust
pub use inner::Foo;

mod inner {
    pub struct Bar;

    pub trait Foo {
        fn foo() -> Bar;
    }
}
