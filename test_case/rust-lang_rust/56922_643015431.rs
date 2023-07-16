rust
// crate a

// this will emit a warning when documenting a
/// [`BrokenLinkA`]
pub struct S;

// crate b

// this will give a warning for `BrokenLinkB`, but not for `BrokenLinkA`
/// Additional context: [`BrokenLinkB`]
pub use a::S;
