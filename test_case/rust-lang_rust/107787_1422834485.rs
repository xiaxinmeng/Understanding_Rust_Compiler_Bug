plain
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error[E0308]: mismatched types
   --> compiler/rustc_codegen_ssa/src/back/symbol_export.rs:442:48
    |
442 |     providers.upstream_monomorphizations_for = upstream_monomorphizations_for_provider;
    |
    |
    = note: expected fn pointer `for<'tcx> fn(TyCtxt<'tcx>, DefId) -> &'tcx Option<&'tcx HashMap<&'tcx rustc_middle::ty::List<rustc_middle::ty::GenericArg<'tcx>>, CrateNum, BuildHasherDefault<FxHasher>>>`
                  found fn item `for<'a> fn(TyCtxt<'a>, DefId) -> Option<&'a HashMap<..., ..., ...>> {upstream_monomorphizations_for_provider}`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_codegen_ssa` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_codegen_ssa` due to previous error
