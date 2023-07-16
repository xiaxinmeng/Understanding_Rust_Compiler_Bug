rust
#![crate_name="foo"]

// @!has "foo/struct.Foo.html"
#[doc(hidden)]
pub struct Foo;

// @has "foo/struct.Bar.html"
pub use crate::Foo as Bar;

// @!has "foo/struct.Baz.html"
#[doc(hidden)]
pub use crate::Foo as Baz;
