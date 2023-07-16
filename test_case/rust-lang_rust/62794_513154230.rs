rust
[cfg!(windows) == false]
Foo { bar: 1, baz: Some(1) }
Foo {
    bar: 1,
    baz: Some(
        1,
    ),
}
