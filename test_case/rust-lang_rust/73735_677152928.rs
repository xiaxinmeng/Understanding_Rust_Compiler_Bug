rust
mod a {
    use std::num::NonZeroUsize;
    extern "C" {
        fn a() -> NonZeroUsize;
    }
}

mod b {
    #[repr(transparent)]
    struct X(NonZeroUsize);
    use std::num::NonZeroUsize;
    extern "C" {
        fn a() -> X;
    }
}
