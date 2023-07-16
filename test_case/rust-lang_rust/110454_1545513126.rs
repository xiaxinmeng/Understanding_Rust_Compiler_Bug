plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0533]: expected unit struct, unit variant or constant, found struct variant `hir::OpaqueTyOrigin::TyAlias`
    |
    |
458 |         hir::OpaqueTyOrigin::TyAlias
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not a unit struct, unit variant or constant
For more information about this error, try `rustc --explain E0533`.
error: could not compile `rustc_hir_analysis` (lib) due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_hir_analysis` (lib test) due to previous error
