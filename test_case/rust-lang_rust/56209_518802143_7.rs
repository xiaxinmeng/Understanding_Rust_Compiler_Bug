rust
pub fn weird<C,I,T,R,F>(collection: &C, indexer: &R, lambda: F) -> Option<T>
where
    C: Index<I>,
    I: Clone,
    R: AsRef<I>,
    F: FnOnce(&<C as Index<I>>::Output) -> Option<T>,
{
    lambda(&collection[indexer.as_ref().clone()])
}
