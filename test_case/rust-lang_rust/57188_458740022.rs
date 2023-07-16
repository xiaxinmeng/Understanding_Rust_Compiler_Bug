rust
#![feature(existential_type)]

struct Baz<'a> {
    source: &'a str,
}

trait Foo<'a> {
    type T: Iterator<Item = Baz<'a>> + 'a;
    fn foo(source: &'a str) -> Self::T;
}

existential type BarFooT<'a>: Iterator<Item = Baz<'a>> + 'a;

struct Bar;
impl<'a> Foo<'a> for Bar {
    type T = BarFooT<'a>;

    fn foo(source: &'a str) -> BarFooT<'a> { // Note: *not* Self::T
        std::iter::once(Baz { source })
    }
}
