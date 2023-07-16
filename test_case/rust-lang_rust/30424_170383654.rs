 rust
fn bar(x: &mut i32) { ... }
fn foo(x: &mut i32) { bar(x); bar(x); }
