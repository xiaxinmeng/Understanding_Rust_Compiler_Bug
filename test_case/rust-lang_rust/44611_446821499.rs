
error: captured variable cannot escape `FnMut` closure body
 --> src/lib.rs:8:28
  |
8 |             .flat_map(|xs| xs.into_iter().flat_map(|_| self.one()))
  |                          - ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returns a reference to a captured variable which escapes the closure body
  |                          |
  |                          inferred to be a `FnMut` closure
  |
  = note: `FnMut` closures only have access to their captured variables while they are executing...
  = note: ...therefore, they cannot allow references to captured variables to escape
