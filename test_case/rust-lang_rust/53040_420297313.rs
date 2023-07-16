
error: captured variable cannot escape `FnMut` closure body
 --> src/main.rs:5:8
  |
5 |     || &mut v;
  |     -- ^^^^^^ creates a reference to the captured variable `v` which escapes the closure body
  |     |
  |     inferred to be a `FnMut` closure
  |
  = note: `FnMut` closures only have access to their captured variables while they are executing
  = note: therefore, they cannot return allow a reference to a captured variable to escape
