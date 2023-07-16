
error[E0596]: cannot borrow immutable local variable `v` as mutable
 --> src/main.rs:3:5
  |
2 |     let v = Vec::new();
  |         - consider changing this to `mut v`
3 |     v.push(0);
  |     ^ cannot borrow mutably
4 |     v.push(0);
  |     ^ cannot borrow mutably

error: aborting due to previous error
