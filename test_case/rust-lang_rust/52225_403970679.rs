rust
macro foo() {} // Should be future-proofed and report an error

#[proc_macro]
fn foo(...) -> ... { ... }
