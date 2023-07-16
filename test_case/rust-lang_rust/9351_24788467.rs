
// Cell-like
impl<T> Mutable<Option<T>> {
    pub fn take(&self) -> Option<T> {
        let mut mptr = self.borrow_mut();
        mptr.get().take()
    }

    pub fn put_back(&self, x: T) {
        let mut mptr = self.borrow_mut();
        assert!(mptr.get().is_none());
        *mptr.get() = Some(x);
    }
}
