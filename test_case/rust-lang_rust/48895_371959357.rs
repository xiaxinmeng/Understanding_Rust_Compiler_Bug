rust
pub trait TraitEngine<'tcx> {
    fn register_predicate_obligation(
        &mut self,
        infcx: &InferCtxt<'a, 'gcx, 'tcx>,
        obligation: PredicateObligation<'tcx>,
    );

    // ... mirror the other fulfillment cx methods as needed ...
}
