rust
// in core::iter

trait IntoIterator {
    type IntoIter: ~const Iterator;
    ...
}

// this would also force the identity implementation to be
impl<I: ~const Iterator> const IntoIterator for I { ... }
