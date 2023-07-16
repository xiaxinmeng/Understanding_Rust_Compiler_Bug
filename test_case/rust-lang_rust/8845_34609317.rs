 rust
struct Foo<'a> {
    f: 'a ||
}

struct Bar<'a> {
    f: &'a Foo<'a>
}

struct Baz<'a> {
    b: &'a Bar<'a>
}

impl<'a> Foo<'a> {
    fn foo(&'a self) -> Bar<'a> {
        Bar { f: self }
    }
}

impl<'a> Bar<'a> {
    fn bar<'a>(&'a self) -> Baz<'a> {
        Baz { b: self }
    }
}
