 rust
#[test] fn foo() {}
mod foo {} //< This causes the test harness's `use` of `foo` to be a hard re-export error.
