rust
fn main() {
    let foo = box Foo{a: Vec::new(), b: Vec::new()};
    let (Foo { a: a, b: b },) = (*foo,);
}
