rust
impl<E,N> Graph<E,N> {
    pub fn read<T,I,F>(&self, indexer: I, lambda: F) -> Option<T>
    where
        Self: Index<I>,
        F: FnOnce(&<Self as Index<I>>::Output) -> Option<T>,
    {
        lambda(&self[indexer])
    }
