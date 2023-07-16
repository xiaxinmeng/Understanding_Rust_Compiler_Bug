
error: captured variable cannot escape `FnMut` closure body
  --> src/main.rs:21:9
   |
13 |     let mut a = A{};
   |         ----- variable defined here
14 |     
15 |     let _c = || {
   |               - inferred to be a `FnMut` closure
...
18 |             f = a.f();
   |                 - variable captured here
...
21 |         f           
   |         ^ returns a reference to a captured variable which escapes the closure body
   |
   = note: `FnMut` closures only have access to their captured variables while they are executing...
   = note: ...therefore, they cannot allow references to captured variables to escape
