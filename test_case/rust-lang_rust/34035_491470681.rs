
error[E0502]: cannot borrow `data` as immutable because it is also borrowed as mutable
 --> src/main.rs:4:47
  |
4 |         let (left, right) = data.split_at_mut(data.len() / 2);
  |                             ---- ------------ ^^^^ immutable borrow occurs here
  |                             |    |
  |                             |    mutable borrow later used by call
  |                             mutable borrow occurs here
