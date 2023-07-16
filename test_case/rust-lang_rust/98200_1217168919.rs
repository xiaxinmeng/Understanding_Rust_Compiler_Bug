rust
enum Foo {
    A,
    B,
    C,
}

fn main() {
    match Foo::A {
        | _foo @ (Foo::A | Foo::B) => {}
//      ^ note the trailing `|`
        Foo::C => {}
    };
}
