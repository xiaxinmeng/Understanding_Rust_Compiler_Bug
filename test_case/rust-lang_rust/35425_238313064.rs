 rust
struct Error<'a, T> {
    inner: marker::PhantomData<&'a RefCell<T>>,
}
