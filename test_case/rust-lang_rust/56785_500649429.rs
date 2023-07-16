
error[E0502]: cannot borrow `x` as immutable because it is also borrowed as mutable
 --> src/main.rs:5:9
  |
4 |         let xm = &mut x;
  |                  ------ mutable borrow occurs here
5 |         &x;
  |         ^^ immutable borrow occurs here
6 |         *xm = 2;
  |         ------- mutable borrow later used here
