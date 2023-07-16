
error: captured variable cannot escape `FnMut` closure body
 --> src/main.rs:4:26
  |
3 | let mut k : usize = 0;
  |     ----- variable defined here
4 |     let mut closure = || &mut k;
  |                        - ^^^^^-
  |                        | |    |
  |                        | |    variable captured here
  |                        | returns a reference to a captured variable which escapes the closure body
  |                        inferred to be a `FnMut` closure
  |
  = note: `FnMut` closures only have access to their captured variables while they are executing...
  = note: ...therefore, they cannot allow references to captured variables to escape
