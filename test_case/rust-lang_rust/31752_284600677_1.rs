text
error: cannot borrow immutable local variable `x` as mutable
 --> x.rs:7:5
  |
6 |     let x = vec![];
  |         - use `mut x` here to make mutable
7 |     x.push(true);
  |     ^ cannot borrow mutably

error: cannot assign to captured outer variable in an `FnMut` closure
 --> x.rs:9:26
  |
9 |     foo(Box::new(move || y = false) as Box<_>);
  |                          ^^^^^^^^^
