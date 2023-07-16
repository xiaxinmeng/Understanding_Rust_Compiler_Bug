rust
impl dyn TraitEngine<'tcx> {
    pub fn new(_tcx: TyCtxt<'_, '_, 'tcx>) -> Box<Self> {
        Box::new(FulfillmentContext::new())
    }
}
