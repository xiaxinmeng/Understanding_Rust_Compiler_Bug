rust
#![feature(generic_associated_types)]

trait Collection {
    type Iter<'a>: IntoIterator;

    fn iter(&self) -> Self::Iter<'_>;
}
