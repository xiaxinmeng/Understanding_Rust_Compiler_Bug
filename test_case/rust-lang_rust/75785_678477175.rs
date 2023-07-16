
error[E0506]: cannot assign to `*x` because it is borrowed
 --> src/lib.rs:3:5
  |
2 |     let y = &*x;
  |             --- borrow of `*x` occurs here
3 |     *x += 1;
  |     ^^^^^^^ assignment to borrowed `*x` occurs here
