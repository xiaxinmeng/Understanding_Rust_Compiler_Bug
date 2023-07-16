
/// Helper trait for creating implementations of `RangeImpl`.
pub trait SampleRange: PartialOrd {
    type T: RangeImpl where T::X == Self;
}

/// Helper trait handling actual range sampling.
pub trait RangeImpl {
    /// The type sampled by this implementation.
    type X: PartialOrd;
    
    /// Construct self.
    /// 
    /// This should not be called directly. `Range::new` asserts that
    /// `low < high` before calling this.
    fn new(low: Self::X, high: Self::X) -> Self;
    
    /// Sample a value.
    fn sample<R: Rng+?Sized>(&self, rng: &mut R) -> Self::X;
}
