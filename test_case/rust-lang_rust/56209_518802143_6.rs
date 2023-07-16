rust
impl<'b, I: Clone, R: AsRef<I>+'b, G: Index<NodeIndex>+Index<EdgeIndex>+Index<I>> Index<&'b R> for G {
    type Output = <Self as Index<I>>::Output;
    fn index<'a>(&'a self, indexer: &'b R) -> Self::Output {
        self.index(indexer.as_ref().clone())
    }
}
