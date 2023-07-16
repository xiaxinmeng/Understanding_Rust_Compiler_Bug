rust
impl<T> Box<MaybeUninit<T>> {
    fn write(self, T) -> Box<T>;
}

impl<T> Arc<MaybeUninit<T>> {
    fn write(self, T) -> Arc<T>;
}

impl<T> Rc<MaybeUninit<T>> {
    fn write(self, T) -> Rc<T>;
}
