
error[E0506]: cannot assign to `*borrow` because it is borrowed
 --> src/main.rs:6:9
  |
5 |         let borrow_borrow = &borrow;
  |                             ------- borrow of `*borrow` occurs here
6 |         *borrow = 1;
  |         ^^^^^^^^^^^ assignment to borrowed `*borrow` occurs here
7 |         let _use = borrow_borrow;
  |                    ------------- borrow later used here
