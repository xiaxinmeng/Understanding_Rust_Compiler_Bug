rust
error: captured variable cannot escape `FnMut` closure body
  --> src/main.rs:36:11
   |
36 |         x(async || {
   |           ^^^^^^^-
   |           |      |
   |           |      inferred to be a `FnMut` closure
   |           returns a reference to a captured variable which escapes the closure body
   |
   = note: `FnMut` closures only have access to their captured variables while they are executing...
   = note: ...therefore, they cannot allow references to captured variables to escape
