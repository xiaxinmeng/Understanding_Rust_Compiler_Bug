rust
pub trait Iterator {
    ...

    fn partition_in_place<'a, T, P>(self, predicate: P) -> usize
    where
        Self: Sized + DoubleEndedIterator<Item = &'a mut T>,
        P: FnMut(&T) -> bool,
        T: 'a {/*...*/}
}
