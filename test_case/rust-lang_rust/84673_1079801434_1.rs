
% rustc +nightly src/main.rs -Z terminal-width=80
error[E0425]: cannot find value `and_then_an_argument` in this scope
 --> src/main.rs:2:43
  |
2 |     a_very_long_function_name_that_pushes(and_then_an_argument);
  |                                           ^^^^^^^^^^^^^^^^^^^^ not found in
  |                                                                this scope
