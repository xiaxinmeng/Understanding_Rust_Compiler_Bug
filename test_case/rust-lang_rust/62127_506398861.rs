rust
// Ok.
 #[tokio_macros::main]
fn foo() {}

// Ok.
use tokio_macros::main as pain;
#[pain]
fn foo() {}

// Not Ok, ambiguity.
use tokio_macros::main;
#[main]
fn foo() {}
