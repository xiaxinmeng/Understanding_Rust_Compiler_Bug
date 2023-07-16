
rustc 1.15.0-nightly (71c06a56a 2016-12-18)
error[E0277]: the trait bound `Box<std::borrow::Borrow<u32>>: std::borrow::Borrow<u32>` is not satisfied
  --> <anon>:10:16
   |
10 |         self.0.borrow()
   |                ^^^^^^ the trait `std::borrow::Borrow<u32>` is not implemented for `Box<std::borrow::Borrow<u32>>`
   |
   = help: the following implementations were found:
   = help:   <Box<T> as std::borrow::Borrow<T>>

error: aborting due to previous error
