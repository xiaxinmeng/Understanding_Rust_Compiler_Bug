
error: captured variable cannot escape `FnMut` closure body
  --> src/lib.rs:9:12
   |
8  |   fn foo(f: &mut dyn FnMut()) {
   |          - variable defined here
9  |       run(|| Box::new(|| {
   |  __________-_^
   | |          |
   | |          inferred to be a `FnMut` closure
10 | |         // dummy lines to push `f` off the screen
11 | |         //
12 | |         //
13 | |         f();
   | |         - variable captured here
14 | |         //
15 | |         //
16 | |     }));
   | |______^ returns a reference to a captured variable which escapes the closure body
   |
   = note: `FnMut` closures only have access to their captured variables while they are executing...
   = note: ...therefore, they cannot allow references to captured variables to escape
