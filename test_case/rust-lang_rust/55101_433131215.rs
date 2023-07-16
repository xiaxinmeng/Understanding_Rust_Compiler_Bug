
error[E0369]: binary operation `==` cannot be applied to type `impl SendEqAlias<i32>`
  --> src/test/run-pass/traits/trait-alias-bounds.rs:56:5
   |
56 |     *x == 22_i32
   |     ^^^^^^^^^^^^
   |
   = note: `impl SendEqAlias<i32>` might need a bound for `std::cmp::PartialEq`

error: aborting due to previous error
