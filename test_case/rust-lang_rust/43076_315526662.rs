
error[E0624]: borrow may still be in use when generator yields
  --> yield-while-local-borrowed.rs:22:18
   |
22 |         let a = &3; //~ ERROR
   |                  ^
23 |         yield();
   |         ------- possible yield occurs here

error[E0624]: borrow may still be in use when generator yields
  --> yield-while-local-borrowed.rs:48:22
   |
48 |             let b = &a; //~ ERROR
   |                      ^
49 |             yield();
   |             ------- possible yield occurs here
