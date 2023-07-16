rust
fn foo() { unsafe {
  fn bar() { /* not considered in unsafe block */ }
} }
