plain
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0308]: mismatched types
    --> compiler/rustc_typeck/src/check/pat.rs:2048:29
     |
2047 |                         match ty {
     |                               -- this expression has type `&TyS<'_>`
2048 |                             ty::Adt(adt_def, _)
     |                             ^^^^^^^^^^^^^^^^^^^ expected struct `TyS`, found enum `rustc_middle::ty::TyKind`
error[E0308]: mismatched types
    --> compiler/rustc_typeck/src/check/pat.rs:2060:29
     |
2047 |                         match ty {
2047 |                         match ty {
     |                               -- this expression has type `&TyS<'_>`
...
2060 |                             ty::Slice(..) | ty::Array(..) => Some((
     |                             ^^^^^^^^^^^^^ expected struct `TyS`, found enum `rustc_middle::ty::TyKind`
error[E0308]: mismatched types
    --> compiler/rustc_typeck/src/check/pat.rs:2060:45
     |
2047 |                         match ty {
2047 |                         match ty {
     |                               -- this expression has type `&TyS<'_>`
...
2060 |                             ty::Slice(..) | ty::Array(..) => Some((
     |                                             ^^^^^^^^^^^^^ expected struct `TyS`, found enum `rustc_middle::ty::TyKind`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_typeck` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
