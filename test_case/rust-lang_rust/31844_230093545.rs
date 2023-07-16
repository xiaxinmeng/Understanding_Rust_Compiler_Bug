 rust
use std::vec::IntoIter as VecIntoIter;

pub trait ClonableIterator: Iterator {
    type ClonableIter;

    fn clonable(self) -> Self::ClonableIter;
}

impl<T> ClonableIterator for T where T: Iterator {
    default type ClonableIter = VecIntoIter<T::Item>;

    default fn clonable(self) -> VecIntoIter<T::Item> {
        self.collect::<Vec<_>>().into_iter()
    }
}

impl<T> ClonableIterator for T where T: Iterator + Clone {
    type ClonableIter = T;

    #[inline]
    fn clonable(self) -> T {
        self
    }
}
