
error: captured variable cannot escape `FnMut` closure body
 --> main_95079_1.rs:3:29
  |
1 | fn foo(s: &str) -> impl Iterator<Item = String> + '_ {
  |        - variable defined here
2 |     None.into_iter()
3 |         .flat_map(move |()| s.chars().map(|c| format!("{}{}", c, s)))
  |                           - -^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |                           | |
  |                           | returns a reference to a captured variable which escapes the closure body
  |                           | variable captured here
  |                           inferred to be a `FnMut` closure
  |
  = note: `FnMut` closures only have access to their captured variables while they are executing...
  = note: ...therefore, they cannot allow references to captured variables to escape
  = help: consider adding 'move' keyword before the parameter of closure in `Map<Chars<'_>, [closure@main_95079_1.rs:3:43: 3:68]>`

error: aborting due to previous error
