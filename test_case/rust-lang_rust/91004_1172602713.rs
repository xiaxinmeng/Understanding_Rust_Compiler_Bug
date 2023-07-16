rust
/// A smart pointer which can can temporarily suspend being smart.
///
/// # Safety
///
/// - `from_raw(into_raw(self))` must return `self` without any change
///   in validity / permissions / provenance / aliasing / etc.
/// - `into_raw(self)` must return a valid pointer to the same object
///   as as dereferencing `self`.
/// - If `Self: DerefMut`, then the pointer returned by `into_raw` must
///   be valid for writes as if through `DerefMut` as well as for reads.
pub unsafe trait Tartare: Deref {
    /// Convert this smart pointer into a raw one.
    ///
    /// It is valid to dereference the raw pointer
    /// as if it were the smart pointer.
    fn into_raw(this: Self) -> NonNull<Self::Target>;
    /// Convert a raw pointer back into a smart one.
    ///
    /// Moves ownership back into `Self`, invalidating all copies of the
    /// raw pointer. Note that this means that it is *not valid* to do
    /// `forget(from_raw(ptr))` and continue using `ptr` for all `Tartare`.
    ///
    /// # Safety
    ///
    /// - `raw` must have come from `Self::into_raw`.
    unsafe fn from_raw(raw: NonNull<Self::Target>) -> Self;
}
