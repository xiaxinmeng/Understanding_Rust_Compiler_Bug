rust
#[test]
fn foo() {}

=>

// The function itself is not changed
#[test]
fn foo() {}

// ... but the import is added.
#[some_marker_for_test_harness]
pub use foo as foo_gensym;
