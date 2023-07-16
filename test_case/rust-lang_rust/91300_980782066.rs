plain
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0412]: cannot find type `HirId` in this scope
  --> compiler/rustc_ty_utils/src/representability.rs:20:23
   |
20 |     SelfRecursive(Vec<HirId>),
   |
help: consider importing this struct
   |
   |
2  | use rustc_hir::HirId;

For more information about this error, try `rustc --explain E0412`.
error: could not compile `rustc_ty_utils` due to previous error
warning: build failed, waiting for other jobs to finish...
