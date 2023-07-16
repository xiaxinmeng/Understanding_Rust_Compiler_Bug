
error[E0503]: cannot use `i[..]` because it was mutably borrowed
 --> main.rs:4:7
  |
4 |     i[i[0] - 1] = 0; // Is rejected
  |     - ^^^^ use of borrowed `i`
  |     |
  |     borrow of `i` occurs here

error: aborting due to previous error
