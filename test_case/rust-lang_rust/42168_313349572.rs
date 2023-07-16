rust
/// Supporting trait for allowing ranges of various different types to be iterators.
#[unstable(feature = "step_trait",
           reason = "recently redesigned",
           issue = "42168")]
pub trait Step: Clone + PartialOrd + Sized {
    /// Returns the number of steps between two step objects. The count is
    /// inclusive of `start` and exclusive of `end`.
    ///
    /// Returns `None` if it is not possible to calculate `steps_between`
    /// without overflow.
    fn steps_between(start: &Self, end: &Self) -> Option<usize>;

    /// “Go forward” (for integers, add) the given number of steps, returning None on overflow.
    fn forward(&self, step_count: usize) -> Option<Self>;

    /// “Go backward” (for integers, subtract) the given number of steps, returning None on overflow.
    fn backward(&self, step_count: usize) -> Option<Self>;

    /// Modify the given inclusive range so that it becomes empty,
    /// for example by setting it to `1...0`.
    fn make_inclusive_range_empty(range: &mut ops::RangeInclusive<Self>);
}
