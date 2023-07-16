
error[E0161]: cannot move a value of type dyn for<'r> std::ops::FnOnce(&'r (dyn for<'s, 't0> std::ops::FnOnce(&'s Trick<'t0>) -> u32 + 'r)) -> u32: the size of dyn for<'r> std::ops::FnOnce(&'r (dyn for<'s, 't0> std::ops::FnOnce(&'s Trick<'t0>) -> u32 + 'r)) -> u32 cannot be statically determined
 --> src/main.rs:5:5
  |
5 |     w(q);
  |     ^
