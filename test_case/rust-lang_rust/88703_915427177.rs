plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: `impl Fn(LocalDefId)` cannot be shared between threads safely
   --> compiler/rustc_middle/src/hir/map/mod.rs:587:45
    |
587 |             par_iter(&items.submodules[..]).for_each(|&sm| par_iter_submodules(tcx, sm, f));
    |                                             ^^^^^^^^ ------------------------------------- within this `[closure@compiler/rustc_middle/src/hir/map/mod.rs:587:54: 587:91]`
    |                                             |
    |                                             `impl Fn(LocalDefId)` cannot be shared between threads safely
    |
    = note: required because it appears within the type `&impl Fn(LocalDefId)`
    = note: required because it appears within the type `&&impl Fn(LocalDefId)`
    = note: required because it appears within the type `[closure@compiler/rustc_middle/src/hir/map/mod.rs:587:54: 587:91]`
    |
    |
584 |         fn par_iter_submodules(tcx: TyCtxt<'_>, module: LocalDefId, f: &impl Fn(LocalDefId) + std::marker::Sync) {

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
