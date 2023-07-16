rust
// crate1
/// Docs in original
pub struct S;

// crate2
/// Docs on re-export
pub use crate1::S;

// how rustdoc sees it: as if _both_ docs were in crate1
/// Docs in original
/// Docs on re-export
pub struct S;
