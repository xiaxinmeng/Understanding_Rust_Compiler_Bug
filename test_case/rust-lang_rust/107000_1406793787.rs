rust
pub struct Foo;
pub trait Bar {}

#[doc(hidden)]
mod bar {
    use crate::{Foo, Bar};

    impl Bar for Foo {} // Is displayed in the docs.
    impl Foo {
        pub fn foo() {} // Is displayed in the docs.
    }
}
