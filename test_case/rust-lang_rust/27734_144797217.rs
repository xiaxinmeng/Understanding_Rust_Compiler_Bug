 rust
impl Utf8Error {
    /// Explain what’s going on after `self.valid_up_to()`
    fn details() -> Utf8ErrorDetails { … }
}

enum Utf8ErrorDetails {
    /// There was an invalid byte sequence.
    ///
    /// In lossy decoding, it should be replaced by a single U+FFFD replacement character.
    /// The given index indicates where in the input to continue decoding.
    /// It is always 1, 2, or 3 more than `Utf8Error::valid_up_to()`.
    Invalid { continue_at_index: usize },

    /// The input (after `Utf8Error::valid_up_to()`) ends with a sequence
    /// that may or may not represent a valid code point
    /// if more subsequent input will be available.
    /// The given length (always 1, 2, or 3) is the maximum number of additional bytes
    /// needed to determine whether it is indeed valid.
    IncompleteSuffix { extra_bytes_needed: usize },
}
