
impl<'tcx> TraitEngine<'tcx> for FulfillmentContext<'tcx> {
    fn register_predicate_obligation(
        &mut self,
        infcx: &InferCtxt<'a, 'gcx, 'tcx>,
        obligation: PredicateObligation<'tcx>,
    ) {
        // forward to the inherent method
        self.register_predicate_obligation(infcx, obligation);
    }
}
