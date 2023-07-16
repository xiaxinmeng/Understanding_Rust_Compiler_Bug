rust
struct Foo {
    a: Vec<u8>,
    b: Vec<u8>
}

fn main() {
    let foo = box Foo{a: Vec::new(), b: Vec::new()};
    let (Foo { a: a, b: b }, _) = (*foo, 0);
}
