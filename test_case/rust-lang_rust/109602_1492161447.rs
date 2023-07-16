rust
#![crate_type = "lib"]

pub mod f {}
pub use unresolved::f;

/// [g]
pub use f as g;
