
error: captured variable cannot escape `FnMut` closure body
  --> src/lib.rs:16:35
   |
16 |         some_iter().filter_map(move |i| self.rooms.get_mut(&i))
   |                                       - ^^^^^^^^^^^^^^^^^^^^^^ returns a reference to a captured variable which escapes the closure body
   |                                       |
   |                                       inferred to be a `FnMut` closure
   |
   = note: `FnMut` closures only have access to their captured variables while they are executing...
   = note: ...therefore, they cannot allow references to captured variables to escape
