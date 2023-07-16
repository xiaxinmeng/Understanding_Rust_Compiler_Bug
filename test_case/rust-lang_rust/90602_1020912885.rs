rust
// in core::iter

trait IntoIterator {
    type IntoIter: Iterator;
    ...
}

// the identity implementation can either be:
impl<I: Iterator> const IntoIterator for I { ... }
// or
impl<I: ~const Iterator> const IntoIterator for I { ... }
