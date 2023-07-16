rust
#[test]
#[cfg(FALSE)] // WARNING: `cfg` is written after `test`, but expanded before `test`, please reorder
fn foo() {}
