
error[E0277]: the trait bound `u32: std::slice::SliceIndex<[i32]>` is not satisfied
 --> src/main.rs:7:24
  |
7 |         println!("{}", vec[i])
  |                        ^^^^^^ slice indices are of type `usize` or ranges of `usize`
  |
  = help: the trait `std::slice::SliceIndex<[i32]>` is not implemented for `u32`
  = note: required because of the requirements on the impl of `std::ops::Index<u32>` for `std::vec::Vec<i32>`
