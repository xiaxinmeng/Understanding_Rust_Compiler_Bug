rust
// crate1
/// [not_found]
pub fn f() {}

// crate2
pub use crate1::f;
