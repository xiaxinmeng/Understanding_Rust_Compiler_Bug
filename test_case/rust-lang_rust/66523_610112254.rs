rust
struct MarkerHolder<T> {
    _marker: std::marker::PhantomData<*const T>
}

fn make_marker() -> MarkerHolder<impl std::fmt::Display> {
    MarkerHolder {
        _marker: std::marker::PhantomData,
    }
}
