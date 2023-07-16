
error: macro expansion ends with an incomplete expression: expected expression
  --> src/main.rs:7:9
   |
2  |       ($($x:tt)*) => {};
   |                      -- in this macro arm
...
7  |       dox(r![
   |  _________^
8  | |         dox(Ok(12)),
9  | |         dox(Err(12)),
10 | |     ]);
   | |_____^ expected expression
