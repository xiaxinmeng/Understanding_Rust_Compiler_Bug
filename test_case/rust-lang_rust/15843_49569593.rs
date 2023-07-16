 rust
struct Foo;
impl Foo {
    fn bar() {}
}

trait Baz {
    fn qux() -> Self;
}
impl Baz for Foo {
    fn qux() -> Foo { Foo }
}

fn main() {
    Foo.bar();

    Foo.qux();
}
