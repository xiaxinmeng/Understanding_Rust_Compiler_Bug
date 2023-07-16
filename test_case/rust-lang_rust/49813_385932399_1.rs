rust
impl DebruijnIndex {
    pub const INNERMOST: DebruijnIndex = DebruijnIndex::new(0);

    /// Returns a DebruijnIndex with the given index.
    pub fn new(index: u32) -> Self {
        DebruijnIndex { index }
    }

    /// Returns the index of the enclosing binder that binds this
    /// region/type. Returns 0 to represent the innermost binder.
    pub fn index(self) -> usize {
        self.index
    }

    /// Returns the resulting index when this value is moved into
    /// `amount` number of new binders. So e.g. if you had
    ///
    ///    for<'a> fn(&'a x)
    ///
    /// and you wanted to change to
    ///
    ///    for<'a> fn(for<'b> fn(&'a x))
    ///
    /// you would need to shift the index for `'a` into 1 new binder.
    pub fn shifted_in(self, amount: u32) -> DebruijnIndex {
        DebruijnIndex { index: self.index + amount }
    }

    /// Update this index in place by shifting it "in" through
    /// `amount` number of binders.
    pub fn shift_in(&mut self, amount: u32) {
        *self = self.shifted(amount);
    }

    /// Returns the resulting index when this value is moved out from
    /// `amount` number of new binders.
    pub fn shifted_out(self, amount: u32) -> DebruijnIndex {
        DebruijnIndex { index: self.index - amount }
    }

    /// Update in place by shifting out from `amount` binders.
    pub fn shift_out(&self, amount: u32) {
        *self = self.shifted(amount);
    }
}
