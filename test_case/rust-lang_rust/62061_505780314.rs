rust
union Foo<T> {
    a: [u8; std::mem::size_of::<T>()],
    b: T,
}
