rust
// inner
pub struct S;

// outer
pub struct T;

/// See also [T].
pub use inner::S;
