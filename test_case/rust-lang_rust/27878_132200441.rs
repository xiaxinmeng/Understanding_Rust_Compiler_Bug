 rust
    /// Retakes ownership of a CString that was transferred to C.
    ///
    /// The only appropriate argument is a pointer obtained by calling
    /// `into_ptr`. The length of the string will be recalculated
    /// using the pointer.
