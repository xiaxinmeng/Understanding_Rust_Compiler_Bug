 rust
impl<F: ?Sized, A> FnMut<A> for Box<F> where F: FnMut<A> {
    ...
}
