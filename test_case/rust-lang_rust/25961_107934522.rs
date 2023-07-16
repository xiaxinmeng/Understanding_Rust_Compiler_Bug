 rust
enum Foo { A, B }

fn bar() -> Foo {
    match Foo::A {
        Foo::A | Foo::B => Foo::B
    }
}
