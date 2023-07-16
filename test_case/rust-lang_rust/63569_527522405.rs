rust
impl<T> Vec<MaybeUninit<T>> {
    /// Transmute this `Vec` of possibly uninitialized data into a normal `Vec`, assuming that the
    /// first `len` elements are properly initialized.
    pub unsafe fn assume_init(self, len: usize) -> Vec<T> {...}
}
/// I'm not 100% sure it's necessary, but define this for symmetry's sake:
impl<T> Vec<T> {
    /// Transmute this `Vec` into a `Vec` of possibly uninitialized data whose length is equal to
    /// the original `Vec`'s capacity.
    pub fn into_uninit(self) -> Vec<MaybeUninit<T>> {...}
}
