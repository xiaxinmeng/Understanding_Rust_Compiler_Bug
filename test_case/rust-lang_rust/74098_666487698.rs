rust
for &module in tcx.hir().krate().modules.keys() {
    let local_def_id = tcx.hir().local_def_id(module);
    tcx.ensure().check_mod_attrs(local_def_id);
};
