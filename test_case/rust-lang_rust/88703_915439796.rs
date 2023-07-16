plain
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking chalk-engine v0.55.0
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
error: ambiguous `+` in a type
   --> compiler/rustc_middle/src/hir/map/mod.rs:584:73
    |
584 |         fn par_iter_submodules(tcx: TyCtxt<'_>, module: LocalDefId, f: &impl Fn(LocalDefId) + Sync) {
    |                                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use parentheses to disambiguate: `(impl Fn(LocalDefId) + Sync)`
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: could not compile `rustc_middle` due to previous error
