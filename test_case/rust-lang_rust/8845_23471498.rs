
use std::cell::Cell;

struct Foo<'self> {
    f: Option<&'self fn()>
}

struct Bar<'self> {
    f: &'self Foo<'self>
}

struct Baz<'self> {
    b: &'self Bar<'self>
}

impl<'self> Foo<'self> {
    fn foo(&'self self) -> Bar<'self> {
        Bar { f: self }
    }
}

impl<'self> Bar<'self> {
    fn bar<'a>(&'a self) -> Baz<'a> {
        Baz { b: self }
    }
}
