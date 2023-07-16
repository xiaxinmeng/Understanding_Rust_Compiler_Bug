rust
#![feature(generic_associated_types)]

trait Collection {
    type Iter<'a>
    where
        Self::Iter<'a>: IntoIterator;

    fn iter<'a>(&'a self) -> Self::Iter<'a>
    where
        Self::Iter<'a>: IntoIterator;
}
