bash
error[E0277]: the trait bound `str: std::marker::Sized` is not satisfied
  --> /home/vagrant/repos/rust/src/test/compile-fail/issue-38954.rs:13:23
   |
13 | fn _test(ref _p: str) {}
   |                       ^ `str` does not have a constant size known at compile-time
   |
   = help: the trait `std::marker::Sized` is not implemented for `str`
