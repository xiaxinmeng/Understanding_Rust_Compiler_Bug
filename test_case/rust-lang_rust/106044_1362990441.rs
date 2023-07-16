rust
macro_rules! m {
    () => {
        struct Foo<'a, T: 'a>(&'a T);
    };
}
m!();
