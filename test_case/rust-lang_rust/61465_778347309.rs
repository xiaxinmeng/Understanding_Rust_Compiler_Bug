
error[E0391]: cycle detected when simplifying constant for the type system `main::C`
 --> src/main.rs:4:1
  |
4 | const C: i32 = B + C; // should be "+ A"
  | ^^^^^^^^^^^^^^^^^^^^^
  |
note: ...which requires simplifying constant for the type system `main::C`...
 --> src/main.rs:4:1
  |
4 | const C: i32 = B + C; // should be "+ A"
  | ^^^^^^^^^^^^^^^^^^^^^
note: ...which requires const-evaluating + checking `main::C`...
 --> src/main.rs:4:1
  |
4 | const C: i32 = B + C; // should be "+ A"
  | ^^^^^^^^^^^^^^^^^^^^^
  = note: ...which requires normalizing `main::C`...
  = note: ...which again requires simplifying constant for the type system `main::C`, completing the cycle
  = note: cycle used when running analysis passes on this crate
