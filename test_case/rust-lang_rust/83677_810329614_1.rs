rust
//! [a][http://example.com](http://example.com)

/// [a][http://example.com](http://example.com)
///
/// [http://example.com]: http://example.com
pub fn bar() {}

/// [a][http://example.com]\(http://example.com)
pub fn foo() {}
