
#![cfg_attr(not(feature = "std"), no_std)]
pub mod task {
    pub mod __internal {
        use crate::task::Waker;
    }
    pub use core::task::Waker;
}
