 Rust
impl<S: DataClone, D: Clone> Clone for ArrayBase<S, D> {
    fn clone(&self) -> ArrayBase<S, D> { /* snip */ }
}
impl<S: DataClone, D: Clone> Clone for ArrayBase<S, D> where Self: Copy {
    fn clone(&self) -> ArrayBase<S, D> { *self }
}
impl<S, D> Clone for ArrayBase<S, D> where Self: Copy {
    fn clone(&self) -> ArrayBase<S, D> { *self }
}

impl<S: Copy, D: Copy> Copy for ArrayBase<S,D> {}
