rust
    #[inline]
    #[rustc_inherit_overflow_checks]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn count(self) -> usize where Self: Sized {
        // Might overflow.
        self.fold(0, |cnt, _| cnt + 1)
    }
