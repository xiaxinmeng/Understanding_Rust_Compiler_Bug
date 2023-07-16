rust
struct TraitsInScope<'tcx> {
    parent: &'tcx TraitsInScope<'tcx>,
    traits: StableVec<DefId>,
}
