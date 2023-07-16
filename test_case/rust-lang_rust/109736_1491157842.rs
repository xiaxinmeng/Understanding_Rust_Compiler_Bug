rust
type LazyInitFn<T> =  impl FnOnce() -> T + Send + ?Sized;

pub struct LazyLock<T> {
    cell: OnceLock<T>,
    init: Cell<Option<LazyInitFn<T>>>,
}

impl<T> LazyLock<T> {
    const fn new(init: LazyInitFn<T>) -> Self { /* ... */ }
}
