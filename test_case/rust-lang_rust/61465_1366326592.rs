
error[E0391]: cycle detected when const-evaluating + checking `main::C`
 --> src/main.rs:4:20
  |
4 | const C: i32 = B + C; // should be "+ A"
  |                    ^
  |
  = note: ...which immediately requires const-evaluating + checking `main::C` again
note: cycle used when simplifying constant for the type system `main::C`
 --> src/main.rs:4:1
  |
4 | const C: i32 = B + C; // should be "+ A"
  | ^^^^^^^^^^^^
