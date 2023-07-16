rust
use std::cell::Cell;
pub struct Foo<'a>(Box<Cell<Foo<'a>>>);
