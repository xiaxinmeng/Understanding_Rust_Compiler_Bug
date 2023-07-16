
#![crate_type = "lib"]

//#[allow(unused_imports)]
use foo::{Foo, Bar};
use foo::{Baz,
          Qux};

mod foo {
    pub struct Foo;
    pub struct Bar;
    pub struct Baz;
    pub struct Qux;
}
