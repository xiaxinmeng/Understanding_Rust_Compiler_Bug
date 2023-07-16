rust
struct Foo<T: ?Sized> {
    bar: [T; Self::LENGTH],
}

impl<T: ?Sized> Foo<T> {
    const LENGTH: usize = 10;
}
