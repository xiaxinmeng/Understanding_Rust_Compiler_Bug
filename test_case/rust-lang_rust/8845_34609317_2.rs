 rust
use std::cell::RefCell;

struct Foo<'a> {
    f: RefCell<&'a 'a ||>
}

struct Bar<'a> {
    f: &'a Foo<'a>
}

impl<'a> Foo<'a> {
    fn foo(&'a self) -> Bar<'a> {
        Bar { f: self }
    }
}
