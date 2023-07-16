
> rustc --version
> rustc 1.19.0-nightly (14f30da61 2017-05-21)
imperio@/Users/imperio/rust/rust > rustc src/test/compile-fail/on-unimplemented/slice-index.rs
error[E0277]: the trait bound `i32: std::slice::SliceIndex<[i32]>` is not satisfied
  --> src/test/compile-fail/on-unimplemented/slice-index.rs:21:5
   |
21 |     x[1i32]; //~ ERROR E0277
   |     ^^^^^^^ the trait `std::slice::SliceIndex<[i32]>` is not implemented for `i32`
   |
   = note: slice indices are of type `usize` or ranges of `usize`
   = note: required because of the requirements on the impl of `std::ops::Index<i32>` for `[i32]`

error[E0277]: the trait bound `std::ops::RangeTo<i32>: std::slice::SliceIndex<[i32]>` is not satisfied
  --> src/test/compile-fail/on-unimplemented/slice-index.rs:24:5
   |
24 |     x[..1i32]; //~ ERROR E0277
   |     ^^^^^^^^^ the trait `std::slice::SliceIndex<[i32]>` is not implemented for `std::ops::RangeTo<i32>`
   |
   = note: slice indices are of type `usize` or ranges of `usize`
   = note: required because of the requirements on the impl of `std::ops::Index<std::ops::RangeTo<i32>>` for `[i32]`

error: aborting due to 2 previous errors
