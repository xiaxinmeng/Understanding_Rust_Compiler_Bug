

error[E0596]: cannot borrow immutable argument `b` as mutable
 --> src/main.rs:4:14
  |
4 |     foo(&mut b)
  |              ^ cannot borrow mutably
help: consider removing the `&mut`, as it is an immutable binding to a mutable reference
4 |     foo(b)
  |         ^
