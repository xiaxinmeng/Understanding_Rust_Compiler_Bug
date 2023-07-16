rust
// foo_1.rs
pub trait Bar {}

// lib.rs
mod foo_1;

/// Hello [`Bar`]. // crash
pub use foo_1::Bar;
