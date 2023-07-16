 rust
extern crate test;

trait Foo<'a> {
    fn foo(bar: &'a Bar) -> Self;
}

struct Bar;

struct Baz<'a> {
    bar: &'a Bar,
}

impl <'a> Foo<'a> for Baz<'a> {
    fn foo(bar: &'a Bar) -> Baz<'a> { Baz { bar: bar } }
}

impl Bar {
    fn foo<'a, T: Foo<'a>>(&'a self) -> T { Foo::foo(self) }
}

fn main() {
    let bar = Bar;
    let baz: Baz = bar.foo();
    test::black_box(baz.bar);
}
