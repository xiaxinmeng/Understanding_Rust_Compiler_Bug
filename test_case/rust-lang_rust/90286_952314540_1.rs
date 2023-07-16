
error[E0507]: cannot move out of `*v` which is behind a shared reference
  --> src/lib.rs:26:13
   |
26 |       let r = v.long_function_with_many_params(
   |               ^ ------------------------------ `v` moved because of this function call
   |
   | note: move occurs because `*v` has type `Type`, which does not implement the `Copy` trait
