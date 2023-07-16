rust
    struct WithLt<'lt> (
        &'lt (),
    );
    
    impl<'db> WithLt<'db> {
        fn check (f: impl FnOnce(*mut &'_ &'db ()))
        {
            Self::check(move |s| f(s))
        }
    }
    