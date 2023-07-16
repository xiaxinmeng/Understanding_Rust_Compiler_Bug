 rust
> cargo new foo
> cargo new bar

// in foo/src/lib.rs
mod sub;

// in foo/src/sub.rs
pub struct Test(i32);

// in bar/Cargo.toml
[dependencies.foo]
path = "../foo"

// in bar/src/lib.rs
extern crate foo;
use foo::sub::Test;
