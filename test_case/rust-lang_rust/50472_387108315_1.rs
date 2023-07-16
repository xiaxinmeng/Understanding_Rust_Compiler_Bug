
warning: unused variable: `y`
 --> src/main.rs:2:26
  |
2 |     ($x:ident) => {{ let $x = 42; }}
  |                          ^^ help: consider using `_y` instead
...
6 |     foo!(y);
  |     -------- in this macro invocation
  |
  = note: #[warn(unused_variables)] on by default
