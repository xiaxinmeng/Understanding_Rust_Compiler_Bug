rust
impl<I: HummockIterator> MergeIteratorNext for OrderedMergeIteratorInner<I> {
    type HummockResultFuture<'a> = impl Future<Output = HummockResult<()>> + 'a;

    fn next_inner(&mut self) -> Self::HummockResultFuture<'_> {
        self.next_inner_inner()
    }
}
