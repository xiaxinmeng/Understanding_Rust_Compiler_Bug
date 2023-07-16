
error[E0425]: cannot find function `does_not_exist` in this scope
 --> src/lib.rs:2:5
  |
2 |     does_not_exist();
  |     ^^^^^^^^^^^^^^ not found in this scope

error[E0282]: type annotations needed
 --> src/lib.rs:6:5
  |
6 |     assert_eq!("x".as_bytes(), &[]);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type
  |
  = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
