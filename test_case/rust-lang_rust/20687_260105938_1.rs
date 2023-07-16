 nocode
error[E0277]: the trait bound `F: std::ops::FnMut<(usize,)>` is not satisfied
 --> <anon>:6:5
  |
6 |     foo(f)
  |     ^^^ the trait `std::ops::FnMut<(usize,)>` is not implemented for `F`
  |
  = help: consider adding a `where F: std::ops::FnMut<(usize,)>` bound
  = note: required by `foo`
