rust
impl<'b, I: Clone, Self: Index<I>, T: AsRef<I> + 'b, E, N> Index<&'b T> for Graph<E,N> {
    type Output = <Self as Index<I>::Output;
    fn index<'a>(&'a self, indexer: &'b T) -> <Self as Index<I>>::Output {
        self.index(indexer.as_ref().clone())
    }
}
