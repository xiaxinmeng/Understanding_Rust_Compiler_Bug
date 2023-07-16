 rust
#[test] fn foo() {}
extern crate foo {} //< This causes the test harness's `use` of `foo` to be a `PRIVATE_IN_PUBLIC` warning.
