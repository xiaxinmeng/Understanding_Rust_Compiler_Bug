rust
impl<'tcx> Key for (Option<VariantIdx>, mir::Field, ty::Const<'tcx>) {
    fn query_crate(&self) -> CrateNum {
        LOCAL_CRATE
    }
    fn default_span(&self, _: TyCtxt<'_, '_, '_>) -> Span {
        DUMMY_SP
    }
}

