rust
#![register_attributes(must_root, another_attr)]

#[must_root]
#[another_attr]
fn foo() {}
