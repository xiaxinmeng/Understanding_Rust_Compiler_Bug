rust
enum Foo {
    A = 0,
    B = Foo::A as isize + 2,
}
