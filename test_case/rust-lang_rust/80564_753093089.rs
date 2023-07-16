rust
struct Foo {
    #[inline]
    bar: String,
}

match foo {
    #[inline]
    _ => {}
}

#[inline]
macro_rules! foo {
    () => {};
}
