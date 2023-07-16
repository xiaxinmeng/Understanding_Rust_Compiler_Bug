
trait Extend {
    /// Extends a collection with exactly one element.
    #[unstable(feature = "extend_one", issue = "none")]
    fn extend_one(&mut self, item: A) {
        self.extend(Some(item));
    }

    /// Reserves capacity in a collection for the given number of additional elements.
    ///
    /// The default implementation does nothing.
    #[unstable(feature = "extend_one", issue = "none")]
    fn extend_reserve(&mut self, _additional: usize) {}
}
