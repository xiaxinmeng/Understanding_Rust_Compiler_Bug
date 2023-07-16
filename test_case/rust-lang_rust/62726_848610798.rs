rust
impl<'a> IoSlice<'a> { // And `IoSliceMut`.
    /// Advance the `IoSlice` by `n` bytes.
    pub fn advance(&mut self, n: usize) {
        // ...
    }

    /// Advance `bufs` by `n` bytes.
    // NOTE: can't (currently) call this as `self: &mut &mut [IoSlice]`.
    pub fn advance_slice(bufs: &mut &mut [IoSlice<'a>], n: usize) {
        // ...
    }
}
