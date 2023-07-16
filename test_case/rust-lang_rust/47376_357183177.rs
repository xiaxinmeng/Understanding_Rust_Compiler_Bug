rust
#![feature(box_patterns)]

struct Foo;

struct Bar {
    a: Foo,
    b: Foo,
}

fn main() {
    let bar = Box::new(Bar { a: Foo, b: Foo });

    let box Bar { a, b } = bar;
}
