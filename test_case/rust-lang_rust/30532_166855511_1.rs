
fn signature(ccx, item: &hir::Item) {
    memoized(..., || {
        ccx.tcx.dep_graph.read(Hir(item_def_id)); // XXX
        compute_signature(ccx, item));
    });
}
