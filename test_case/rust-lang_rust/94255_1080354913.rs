plain
    Checking cargo_metadata v0.14.0
    Checking rustfix v0.5.1
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
    Checking clippy_lints v0.1.61 (/checkout/src/tools/clippy/clippy_lints)
error[E0308]: `?` operator has incompatible types
  --> src/tools/clippy/clippy_lints/src/matches/overlapping_arms.rs:37:47
   |
37 |                         None => miri_to_const(ty.numeric_min_val(cx.tcx)?)?,
   |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `rustc_middle::ty::Const`, found `u128`
   |
   = note: `?` operator cannot convert from `u128` to `rustc_middle::ty::Const<'_>`

error[E0308]: `?` operator has incompatible types
  --> src/tools/clippy/clippy_lints/src/matches/overlapping_arms.rs:41:47
   |
41 |                         None => miri_to_const(ty.numeric_max_val(cx.tcx)?)?,
   |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `rustc_middle::ty::Const`, found `u128`
   |
   = note: `?` operator cannot convert from `u128` to `rustc_middle::ty::Const<'_>`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `clippy_lints` due to 2 previous errors
Build completed unsuccessfully in 0:03:10
