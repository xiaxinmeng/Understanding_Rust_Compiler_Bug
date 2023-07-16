plain
   Compiling gsgdt v0.1.2
error: panic message is not a string literal
  --> compiler/rustc_ast/src/mut_visit.rs:31:34
   |
31 |         assert!(self.len() == 1, err);
   |
   |
   = note: `-D non-fmt-panic` implied by `-D warnings`
   = note: this is no longer accepted in Rust 2021
help: add a "{}" format string to Display the message
   |
31 |         assert!(self.len() == 1, "{}", err);

   Compiling rls-data v0.19.1
error: aborting due to previous error

