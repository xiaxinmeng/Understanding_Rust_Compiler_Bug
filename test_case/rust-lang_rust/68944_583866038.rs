rust
query index_hir(_: CrateNum) -> &'tcx map::IndexedHir<'tcx> {
    eval_always
    no_hash
    desc { "index HIR" }
}

query hir_module_items(key: DefId) -> &'tcx hir::ModuleItems {
    eval_always
}

query hir_owner(key: DefId) -> &'tcx HirOwner<'tcx> {
    eval_always
}

query hir_owner_items(key: DefId) -> &'tcx HirOwnerItems<'tcx> {
    eval_always
}

query all_local_trait_impls(key: CrateNum) -> &'tcx BTreeMap<DefId, Vec<hir::HirId>> {
    desc { "local trait impls" }
}
