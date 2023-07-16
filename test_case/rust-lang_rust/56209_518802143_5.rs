rust
impl<E,N> Graph<E,N> {

    fn read_ref<R,T,I,F>(&self, index: &R, lambda: F) - Option<T>
    where
        Self: Index<I>,
        I: Clone,
        R: AsRef<I>,
        F: FnOnce(&<Self as Index<I>>::Output) -> Option<T>,
    {
        lambda(& self[index.as_ref().clone()])
    }
}
