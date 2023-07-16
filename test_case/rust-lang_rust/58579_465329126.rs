 rust
// arm/mod.rs

#[cfg(not(dox))] // <- ADD
pub use super::acle::*;

// ADD
#[cfg(dox)]
pub use super::acle::{dmb, dsb, /* plus dozens of other funcions */};
