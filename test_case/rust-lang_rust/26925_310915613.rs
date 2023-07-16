rust
use std::cell::RefCell;
#[derive(Default)]
pub struct LazyCell<T> {
    inner: RefCell<Option<T>>,
}
