plain
    Checking clippy_lints v0.1.61 (/checkout/src/tools/clippy/clippy_lints)
error: unused import: `miri_to_const`
 --> src/tools/clippy/clippy_lints/src/matches/overlapping_arms.rs:1:57
  |
1 | use clippy_utils::consts::{constant, constant_full_int, miri_to_const, FullInt};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
error[E0308]: `match` arms have incompatible types
  --> src/tools/clippy/clippy_lints/src/matches/overlapping_arms.rs:37:33
   |
   |
35 |                       let lhs_val = match lhs {
   |  ___________________________________-
36 | |                         Some(lhs) => constant(cx, cx.typeck_results(), lhs)?.0.int_value(cx, ty)?,
   | |                                      ------------------------------------------------------------ this is found to be of type `FullInt`
37 | |                         None => ty.numeric_min_val(cx.tcx),
   | |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `FullInt`, found enum `std::option::Option`
   | |_____________________- `match` arms have incompatible types
   |
   |
   = note: expected type `FullInt`

error[E0308]: `match` arms have incompatible types
  --> src/tools/clippy/clippy_lints/src/matches/overlapping_arms.rs:41:33
   |
   |
39 |                       let rhs_val = match rhs {
   |  ___________________________________-
40 | |                         Some(rhs) => constant(cx, cx.typeck_results(), rhs)?.0.int_value(cx, ty)?,
   | |                                      ------------------------------------------------------------ this is found to be of type `FullInt`
41 | |                         None => ty.numeric_max_val(cx.tcx),
   | |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `FullInt`, found enum `std::option::Option`
   | |_____________________- `match` arms have incompatible types
   |
   |
   = note: expected type `FullInt`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `clippy_lints` due to 3 previous errors
Build completed unsuccessfully in 0:03:40
