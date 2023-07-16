rust
#![feature(generic_associated_types)]

trait Collection {
    type Iter
    where
        Self::Iter: IntoIterator,
        <Self::Iter as IntoIterator>::Item: std::fmt::Debug;

    fn iter(&self) -> Self::Iter
    where
        Self::Iter: IntoIterator,
        <Self::Iter as IntoIterator>::Item: std::fmt::Debug;
}
