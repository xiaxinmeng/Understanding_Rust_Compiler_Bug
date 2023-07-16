Rust
union Foo {
    a: isize,
    b: &T,
}
enum Bar {
    Boo = [Foo { b: () }.a][3],
}
