rust
pub struct TestLazyLock<T> {
    cell: OnceLock<T>,
    init: Cell<Option<fn() -> T>>,
}

impl<T> TestLazyLock<T> {
    const fn new(init: fn() -> T) -> Self {
        Self { cell: OnceLock::new(), init: Cell::new(Some(init)) }
    }
}
