rust
match foo {
    Foo::A(_) => (),
    // ...
    Foo::Void(empty, _) => match empty {},
    //               ^
    // phantom data
}
