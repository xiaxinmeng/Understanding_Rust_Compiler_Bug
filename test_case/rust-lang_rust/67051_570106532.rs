
error[E0277]: the type `[i64]` cannot be indexed by `i64`
  --> src/main.rs:10:5
   |
10 |     vec[index] = get_vec(&vec) + get_vec(&vec);
   |     ^^^^^^^^^^ slice indices are of type `usize` or ranges of `usize`
   |
   = help: the trait `std::slice::SliceIndex<[i64]>` is not implemented for `i64`
   = note: required because of the requirements on the impl of `std::ops::Index<i64>` for `std::vec::Vec<i64>`
