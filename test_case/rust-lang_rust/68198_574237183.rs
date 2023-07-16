rust
crate core {
    pub mod lazy {
        pub struct OnceCell;
        pub struct Lazy;
    }
}

crate std {
    pub mod lazy {
        pub use core::lazy::*;

        pub struct SyncOnceCell;
        pub struct SyncLazy;
    }
}
