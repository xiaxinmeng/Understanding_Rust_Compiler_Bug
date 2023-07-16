rust
use arrayvec::ArrayString;

#[derive(PartialEq,Eq)]
enum Foo {
    A,
}

#[derive(PartialEq, Eq)]
enum Bar {
    Foo(Foo),
    #[allow(dead_code)]
    Other(ArrayString<64>)
}

impl Bar {
    const A: Bar = Bar::Foo(Foo::A);
}

fn main() {
    match Bar::A {
        Bar::A => {},
        _ => {}
    }
}
