rust
enum Foo {
    A = "" + 1
}

enum Bar {
    A = Foo::A as isize
}
