rust
#[cfg(stage0)]
fn foo() {}
#[cfg(not(stage0))]
const fn foo() {}
