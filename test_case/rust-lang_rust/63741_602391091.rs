rust
use std::mem; // `self` imports are only allowed within a { } list
fn main() { mem::drop(42); } // use of undeclared type or module `mem`
