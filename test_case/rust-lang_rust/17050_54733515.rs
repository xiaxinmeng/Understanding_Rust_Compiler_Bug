 rust
enum Foo {
    A, B, C
}
enum Bar {
     X(Foo), Y
}

enum Baz {
      S(Bar), T(int)
}
