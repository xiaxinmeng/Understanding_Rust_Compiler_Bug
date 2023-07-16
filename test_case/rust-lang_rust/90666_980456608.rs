rust
impl<T> Rc<T> {
    pub fn new_cyclic(f: impl FnOnce(&Weak<T>) -> T) -> Rc<T>;
}

impl<T> Arc<T> {
    pub fn new_cyclic(f: impl FnOnce(&Weak<T>) -> T) -> Arc<T>;
}
