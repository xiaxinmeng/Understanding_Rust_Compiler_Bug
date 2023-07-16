rust
    /// Returns `true` if the address of the two reference counted objects (the
    /// `Weaks`) are equal.
    ///
    /// # Notes
    ///
    /// Since this compares addresses it means that `Weak::new()` will always
    /// equal each other.
