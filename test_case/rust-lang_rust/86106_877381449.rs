rust
    #[inline]
    #[rustc_const_stable(feature = "const_vec_new", since = "1.39.0")]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[cfg(bootstrap)]
    pub const fn new() -> Self {
        Vec { buf: RawVec::NEW, len: 0 }
    }

    #[inline]
    #[rustc_const_stable(feature = "const_vec_new", since = "1.39.0")]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[cfg(not(bootstrap))]
    pub const fn new() -> Self {
        Vec { buf: RawVec::new(), len: 0 }
    }
