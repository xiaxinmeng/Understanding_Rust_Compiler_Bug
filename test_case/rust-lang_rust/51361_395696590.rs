rust
enum Foo {
    A = 5,
    B = 42,
}
enum Bar {
    C = 42,
    D = 99,
}
union Union {
    foo: &'static Foo,
    bar: &'static Bar,
    usize: &'static usize,
}
static FOO: (&Foo, &Bar, usize) = (
    Union { usize: &FOO.2 }.foo,
    Union { usize: &FOO.2 }.bar,
    42,
);
