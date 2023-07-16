plain
    Checking clippy_lints v0.1.61 (/checkout/src/tools/clippy/clippy_lints)
error[E0308]: mismatched types
  --> src/tools/clippy/clippy_lints/src/matches/overlapping_arms.rs:37:47
   |
37 |                         None => miri_to_const(ty.numeric_min_val(cx.tcx))?,
   |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `rustc_middle::ty::Const`, found enum `std::option::Option`
   = note: expected struct `rustc_middle::ty::Const<'_>`
                found enum `std::option::Option<u128>`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> src/tools/clippy/clippy_lints/src/matches/overlapping_arms.rs:41:47
   |
41 |                         None => miri_to_const(ty.numeric_max_val(cx.tcx))?,
   |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `rustc_middle::ty::Const`, found enum `std::option::Option`
   = note: expected struct `rustc_middle::ty::Const<'_>`
                found enum `std::option::Option<u128>`

For more information about this error, try `rustc --explain E0308`.
