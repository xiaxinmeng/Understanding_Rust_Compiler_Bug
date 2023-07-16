plain
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error[E0252]: the name `DefKind` is defined multiple times
  --> compiler/rustc_typeck/src/check/check.rs:31:5
   |
9  | use rustc_hir::def::{DefKind, Res};
   |                      ------- previous import of the type `DefKind` here
31 | use rustc_hir::def::DefKind;
31 | use rustc_hir::def::DefKind;
   |     ^^^^^^^^^^^^^^^^^^^^^^^ `DefKind` reimported here
   |
   = note: `DefKind` must be defined only once in the type namespace of this module
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error: unused import: `rustc_hir::def::DefKind`
  --> compiler/rustc_typeck/src/check/check.rs:31:5
  --> compiler/rustc_typeck/src/check/check.rs:31:5
   |
31 | use rustc_hir::def::DefKind;
   |     ^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`
For more information about this error, try `rustc --explain E0252`.
error: could not compile `rustc_typeck` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_typeck` due to 2 previous errors
