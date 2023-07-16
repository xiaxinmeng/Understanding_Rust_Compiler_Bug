rust
#![feature(type_alias_impl_trait, associated_type_defaults)]

use std::fmt::Debug;

struct Foo;

trait Bar {
    type T: Debug = impl Debug;
    fn bar() -> Self::T {
        ()
    }
}

impl Bar for Foo {
    fn bar() -> Self::T {
        ()
    }
}
