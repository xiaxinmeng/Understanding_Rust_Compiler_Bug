rust
error[E0596]: cannot borrow immutable item `*TAB[..]` as mutable
  --> test.rs:16:5
   |
16 |     TAB[0].iter_mut();
   |     ^^^^^^ cannot borrow as mutable
   |
   = note: Value not mutable causing this error: `TAB`

error: aborting due to previous error
