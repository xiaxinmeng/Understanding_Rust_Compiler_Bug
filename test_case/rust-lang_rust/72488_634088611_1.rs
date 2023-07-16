rust
impl<T> Foo<T> {
    const _: () = assert!(TypeId::of<T>() == TypeId::of<usize>());
}
