rust
use std::fmt;

#[derive(Debug)]
pub struct Foo;

impl fmt::Display for Foo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <fmt::Debug>::fmt(self, f)  // bare_trait_objects
    }
}
