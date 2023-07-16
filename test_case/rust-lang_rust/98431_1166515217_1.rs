text
error[E0308]: mismatched types
 --> src/main.rs:1:7
  |
1 | fn _f(&mut _a: i32) {}
  |       ^^^^^^^  --- expected due to this
  |       |
  |       expected `i32`, found `&mut _`
  |
  = note:           expected type `i32`
          found mutable reference `&mut _`
help: consider removing `&mut` from the pattern or using `mut` to declare variable as mutable
  |
1 - fn _f(&mut _a: i32) {}
1 + fn _f(_a: i32) {}
  |
1 | fn _f(mut _a: i32) {}
  |       ~~~
