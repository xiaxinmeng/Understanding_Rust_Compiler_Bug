rust
// in alloc
impl<T, U: ?Sized> From<T> for Box<U> where T: Unsize<U>;
