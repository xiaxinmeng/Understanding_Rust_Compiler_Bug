 rust
pub trait FromIterator<A> {
    /// Build a container with elements from an external iterator.
    ///
    /// `cap_hint` is a hint as to what the capacity of the collection should be.
    /// It is usually, but not always, the lower size bound of the iterator.
    fn from_iter<T: Iterator<A>>(iterator: T, cap_hint: uint) -> Self;
}

pub trait Iterator<A> {
    // ...
    fn collect<B: FromIterator<A>>(&mut self) -> B {
        let (lower, _) = self.size_hint();
        FromIterator::from_iter(self.by_ref(), lower)
    }
}
