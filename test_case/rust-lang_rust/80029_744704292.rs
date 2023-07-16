
impl<T, A> Arc<T, A> {
  fn from_repr(repr: Box<ArcRepr<T, A>>) -> Arc<T, A> {
    ...
  }
}
