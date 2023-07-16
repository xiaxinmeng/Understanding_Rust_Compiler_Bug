
error[E0596]: cannot borrow immutable item `x` as mutable
 --> src/main.rs:5:10
  |
5 |     || { &mut x; };
  |          ^^^^^^ cannot borrow as mutable
  |
  = note: Value not mutable causing this error: `x`

error[E0596]: cannot borrow immutable item `x` as mutable
 --> src/main.rs:5:5
  |
5 |     || { &mut x; };
  |     ^^^^^^^^^^^^^^ cannot borrow as mutable
