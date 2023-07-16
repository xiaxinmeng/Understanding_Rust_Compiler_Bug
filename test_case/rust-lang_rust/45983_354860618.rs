
error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements
 --> src/main.rs:7:27
  |
7 |     give_any(|y| x = Some(y));
  |                           ^
  |
note: first, the lifetime cannot outlive the anonymous lifetime #2 defined on the body at 7:14...
 --> src/main.rs:7:14
  |
7 |     give_any(|y| x = Some(y));
  |              ^^^^^^^^^^^^^^^
note: ...so that expression is assignable (expected &(), found &())
 --> src/main.rs:7:27
  |
7 |     give_any(|y| x = Some(y));
  |                           ^
note: but, the lifetime must be valid for the block suffix following statement 0 at 6:5...
 --> src/main.rs:6:5
  |
6 | /     let x = None;
7 | |     give_any(|y| x = Some(y));
8 | | }
  | |_^
note: ...so that variable is valid at time of its declaration
 --> src/main.rs:6:9
  |
6 |     let x = None;
  |         ^
