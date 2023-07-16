
error: captured variable cannot escape `FnMut` closure body            ] 21/36: core                                                                                    
  --> src/libcore/fmt/builders.rs:17:13
   |
12 |         Self::wrap_buf_test(fmt, move |buf| {
   |                                           - inferred to be a `FnMut` closure
...
17 |             slot.as_mut().unwrap()
   |             ^^^^^^^^^^^^^^^^^^^^^^ returns a reference to a captured variable which escapes the closure body
   |
   = note: `FnMut` closures only have access to their captured variables while they are executing...
   = note: ...therefore, they cannot allow references to captured variables to escape
