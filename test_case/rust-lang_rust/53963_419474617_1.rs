rust
// crate "B"
extern crate a;

pub fn bar() { a::foo(); }
