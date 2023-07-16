rust
pub fn g() {}
/// Link to [crate::g]
pub fn f() {}

// outer
pub use inner::g;
pub use inner::f;
