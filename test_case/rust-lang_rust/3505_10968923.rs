
pub mod proj {

    // This is the sole publicly-visible function: proj::entry_fn.
    pub use impl::entry_fn;

    mod impl {
        // All your implementation items are in here.
        pub use stuff::*;
        pub use things::*;
        pub mod stuff;
        pub mod things;

        pub fn entry_fn() { ... }
        pub fn other_fn1() { ... }
    }
}
