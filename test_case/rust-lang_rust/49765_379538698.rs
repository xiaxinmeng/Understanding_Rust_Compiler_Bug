rust
enum Foo {
    Bar = Foo::Baa as isize + 1,
    Baa = 42,
}
