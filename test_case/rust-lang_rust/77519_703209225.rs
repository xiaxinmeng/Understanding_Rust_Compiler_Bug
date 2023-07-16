rust
// crate 1
/// Here
/// have
/// docs
pub struct S;

// crate 2
/// Have
/// more
/// dox
pub use crate1::S;
