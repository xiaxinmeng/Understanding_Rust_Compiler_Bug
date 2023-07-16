 rust
enum TransItem<'tcx> {
    Fn {
        node_id: NodeId,
        substs: &'tcx Substs<'tcx>,
        original_definition: DefId,
        method_dispatch_cache: FnvHashMap<CallKey, (DefId, &'tcx Substs<'tcx>)>
    },
    ...
}
