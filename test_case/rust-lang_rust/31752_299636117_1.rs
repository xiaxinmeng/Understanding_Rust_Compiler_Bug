
error: cannot assign to captured outer variable in an `FnMut` closure
 --> test.rs:7:26
  |
7 |     foo(Box::new(move || y = false) as Box<_>);
  |                          ^^^^^^^^^

error: aborting due to previous error
