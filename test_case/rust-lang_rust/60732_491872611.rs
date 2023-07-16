rust
enum Foo {
    A(&'static i32),
    B,
}
enum Bar {
    A(&'static i32) = 0,
    B = 1,
}
assert_eq!(std::mem::size_of::<Foo>() == std::mem::size_of::<usize>());
assert_eq!(std::mem::size_of::<Bar>() > std::mem::size_of::<usize>());
