rust
enum Foo {
    A = 0,
    B = {let v = Foo::A; v as isize + 2},
}
