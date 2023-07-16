 rust
        pub unsafe const fn new(inner: T) -> NonZero<T> {
            debug_assert!(!inner.is_zero());
            NonZero(inner)
        }
