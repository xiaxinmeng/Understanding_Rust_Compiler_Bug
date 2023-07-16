
error[E0310]: the parameter type `F` may not live long enough
  --> src/lib.rs:9:1
   |
9  | / {
10 | | }
   | |_^
   |
   = help: consider adding an explicit lifetime bound `F: 'static`...

error: aborting due to previous error

For more information about this error, try `rustc --explain E0310`.
