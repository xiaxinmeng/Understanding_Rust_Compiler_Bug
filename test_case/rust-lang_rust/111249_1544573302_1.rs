rust
#![crate_name="foo"]

// This assertion fails. The file gets generated.
// @!has "foo/foo/index.html"
#[doc(hidden)]
pub mod foo {}

// This assertion passes.
// @has "foo/bar/index.html"
pub use crate::foo as bar;

// This assertion also fails. The file gets generated.
// @!has "foo/baz/index.html"
#[doc(hidden)]
pub use crate::foo as baz;
