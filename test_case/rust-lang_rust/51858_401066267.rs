rust
crate struct ExplicitPredicatesMap<'tcx> {
  map: FxHashMap<DefId, RequiredPredicates<'tcx>>
}
