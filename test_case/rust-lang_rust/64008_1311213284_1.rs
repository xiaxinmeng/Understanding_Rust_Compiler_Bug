
error: captured variable cannot escape `FnMut` closure body
 --> src/lib.rs:7:9
  |
4 |     bar: &'a X,
  |     --- variable defined here
5 | ) -> impl Iterator<Item = ()> + 'a {
6 |     Some(()).iter().flat_map(move |()| {
  |                                      - inferred to be a `FnMut` closure
7 |         Some(()).iter().map(|()| { bar; })
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^---^^^^
  |         |                          |
  |         |                          variable captured here
  |         returns a reference to a captured variable which escapes the closure body
  |
  = note: `FnMut` closures only have access to their captured variables while they are executing...
  = note: ...therefore, they cannot allow references to captured variables to escape

