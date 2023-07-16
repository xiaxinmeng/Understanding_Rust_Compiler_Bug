
error[E0502]: cannot borrow `x` as immutable because it is also borrowed as mutable
 --> src/main.rs:4:15
  |
4 |     x.get_mut(x.len());
  |     - ------- ^ immutable borrow occurs here
  |     | |
  |     | mutable borrow later used by call
  |     mutable borrow occurs here

error: aborting due to previous error
