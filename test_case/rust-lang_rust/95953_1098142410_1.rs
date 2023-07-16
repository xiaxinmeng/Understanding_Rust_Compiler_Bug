rust
error[E0506]: cannot assign to `x` because it is borrowed
 --> test.rs:5:5
  |
3 |     let r = &x;
  |             -- borrow of `x` occurs here
4 |     let a = [r; 0];
5 |     x = 5;
  |     ^^^^^ assignment to borrowed `x` occurs here
6 |     let _b = a;
  |              - borrow later used here

error: aborting due to previous error; 1 warning emitted
