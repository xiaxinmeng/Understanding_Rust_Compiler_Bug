rust
        #[allow(invalid_value, clippy::uninit_assumed_init)]
        let mut inner: Inner = mem::MaybeUninit::uninit().assume_init();
