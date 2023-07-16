 rust
#![crate_type = "lib"]
#![feature(globs)]

pub use a::Foo;

mod a {
    pub struct Foo;
}

mod b {
    pub use super::*;
}
