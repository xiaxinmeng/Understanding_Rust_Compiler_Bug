
error[E0310]: the parameter type `T` may not live long enough
 --> test.rs:4:5
  |
4 |     foo: &'static T
  |     ^^^^^^^^^^^^^^^
  |
  = help: consider adding an explicit lifetime bound `T: 'static`...
note: ...so that the reference type `&'static T` does not outlive the data it points at
 --> test.rs:4:5
  |
4 |     foo: &'static T
  |     ^^^^^^^^^^^^^^^

error: aborting due to previous error(s)
