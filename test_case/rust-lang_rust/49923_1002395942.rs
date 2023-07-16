
error[E0499]: cannot borrow `t` as mutable more than once at a time
 --> src/main.rs:8:11
  |
8 |         f(&mut t);
  |           ^^^^^^ `t` was mutably borrowed here in the previous iteration of the loop
