rust
enum AllocId<'tcx> {
    Static(DefId),
    Function(Instance<'tcx>),
    Local(u64),
}
