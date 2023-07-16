 rust
impl<T> RefCell<T> {
    /// Take a borrow that's "permanent", i.e. no further mutable borrows
    /// will be permitted after this call.
    ///
    /// This provides a borrow that lasts as long as the RefCell (unlike
    /// .borrow(), which is restricted to the scope of the returned Ref).
    fn perma_borrow<'a>(&'a self) -> &'a T {
        // take an immutable borrow, setting the BorrowFlag without ever
        // cleaning it up
        match self.borrow() {
            WRITING => fail!(...),
            borrow => {
                self.borrow.set(borrow + 1);
                unsafe { &*self.value.get() }
            }
        }
    }
}
