plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error[E0433]: failed to resolve: could not find `AssocTyConstraintKind` in `ast`
   --> compiler/rustc_resolve/src/late/diagnostics.rs:621:26
    |
621 |                 let ast::AssocTyConstraintKind::Bound { bounds } = &constraint.kind else {
    |                          ^^^^^^^^^^^^^^^^^^^^^ could not find `AssocTyConstraintKind` in `ast`
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
For more information about this error, try `rustc --explain E0433`.
error: could not compile `rustc_resolve` due to previous error
warning: build failed, waiting for other jobs to finish...
