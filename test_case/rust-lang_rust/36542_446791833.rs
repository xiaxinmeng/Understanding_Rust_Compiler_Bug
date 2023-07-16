
error[E0596]: cannot borrow immutable argument `b` as mutable
 --> src/main.rs:2:18
  |
2 |     let x = &mut b;
  |                  ^ cannot borrow mutably
help: consider removing the `&mut`, as it is an immutable binding to a mutable reference
  |
2 |     let x = b;
  |             ^
