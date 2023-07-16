rust
/// Returns the highest element whose key is below the given bound.
fn upper_bound<Q>(&self, bound: Bound<&Q>) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized;

/// Returns the lowest element whose key is above the given bound.
fn lower_bound<Q>(&self, bound: Bound<&Q>) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized;
