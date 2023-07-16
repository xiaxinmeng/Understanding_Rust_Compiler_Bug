rust
impl ExplicitPredicatesMap<'tcx> {
    crate fn explicit_predicates_of(&mut self, tcx: TyCtxt<'_, '_, 'tcx>, def_id: DefId) -> &RequiredPredicates<'tcx> {
        self.map.entry(def_id).or_insert_with(|| {
            let predicates = if def_id.is_local() {
                tcx.explicit_predicates_of(def_id)
            } else {
                tcx.predicates_of(def_id)
            };
            let required_predicates = {
                // process predicates and convert to `RequiredPredicates` entry, see below
            };
            required_predicates
        }
    }
}
