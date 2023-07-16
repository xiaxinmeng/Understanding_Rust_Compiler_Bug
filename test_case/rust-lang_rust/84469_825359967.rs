plain
    Checking clippy_utils v0.1.53 (/checkout/src/tools/clippy/clippy_utils)
    Checking url v2.1.1
    Checking cargo_metadata v0.12.0
    Checking clippy_lints v0.1.53 (/checkout/src/tools/clippy/clippy_lints)
error[E0599]: no method named `name_str` found for enum `rustc_hir::PrimTy` in the current scope
  --> src/tools/clippy/clippy_lints/src/from_str_radix_10.rs:89:63
   |
89 |                     format!("{}.parse::<{}>()", sugg, prim_ty.name_str()),
   |                                                               ^^^^^^^^ method not found in `rustc_hir::PrimTy`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: could not compile `clippy_lints`
