 Rust
pub unsafe trait Data : Sized {
    type Elem;
    ...
}
pub unsafe trait DataClone : Data {
    unsafe fn clone_with_ptr(&self, ptr: *mut Self::Elem) -> (Self, *mut Self::Elem);
}

impl<S: DataClone, D: Clone> Clone for ArrayBase<S, D> {
    fn clone(&self) -> ArrayBase<S, D> { /* snip */ }
}

impl<S: DataClone + Copy, D: Copy> Copy for ArrayBase<S, D> {}
