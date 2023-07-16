 rust
struct Foo {
    a: Vec<u8>,
    b: Vec<u8>
}

fn move<T>(x: T) -> T { x }

fn main() {
    let foo = box Foo{a: Vec::new(), b: Vec::new()};
    let Foo { a: a, b: b } = *move(foo);
}
