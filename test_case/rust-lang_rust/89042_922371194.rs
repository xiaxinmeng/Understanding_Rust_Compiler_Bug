rust
enum Foo {
    A, B,
    #[doc(hidden)]
    C,
}

// Different crate
match Foo::A {
    Foo::A => {}
    Foo::B => {}
}

match Foo::A {
    Foo::A => {}
    Foo::C => {}
}
