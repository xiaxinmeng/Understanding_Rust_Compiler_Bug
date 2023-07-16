 rust
impl<F: ?Sized, A> FnOnce<A> for Box<F> where F: FnOnce<A> {
    ...
}
