rust
    /// Returns a mutable reference to the of the element that the cursor is
    /// currently pointing to.
    ///
    /// This can be used to modify the key, but you must ensure that the
    /// `BTreeMap` invariants are maintained. Specifically:
    ///
    /// * The key must remain unique within the tree.
    /// * The key must remain in sorted order with regards to other elements in
    ///   the tree.
    unsafe fn key_mut_unchecked(&mut self) -> Option<&mut K>;
