 rust
#![crate_type = "lib"]

use foo::{Foo, Bar, Baz, Qux};

mod foo {
    pub struct Foo;
    pub struct Bar;
    pub struct Baz;
    pub struct Qux;
}
