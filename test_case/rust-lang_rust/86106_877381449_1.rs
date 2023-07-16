rust
    #[rustc_allow_const_fn_unstable(const_fn)]
    #[cfg_attr(not(bootstrap), rustc_const_unstable(feature="rawvec_const_new_fake_stable", issue="99999"))]
    pub const fn new() -> Self {
        Self::new_in(Global)
    }
