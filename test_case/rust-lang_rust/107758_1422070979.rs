`
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error[E0308]: mismatched types
   --> compiler/rustc_codegen_ssa/src/back/symbol_export.rs:442:48
    |
442 |     providers.upstream_monomorphizations_for = upstream_monomorphizations_for_provider;
    |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected reference, found enum `Option`
    |
    = note: expected fn pointer `for<'tcx> fn(TyCtxt<'tcx>, DefId) -> &'tcx Option<&'tcx HashMap<&'tcx rustc_middle::ty::List<rustc_middle::ty::GenericArg<'tcx>>, CrateNum, BuildHasherDefault<FxHasher>>>`
                  found fn item `for<'a> fn(TyCtxt<'a>, DefId) -> Option<&'a HashMap<..., ..., ...>> {upstream_monomorphizations_for_provider}`

