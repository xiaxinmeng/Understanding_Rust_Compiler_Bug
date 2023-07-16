rust
pub type CanonicalTy<'gcx> = Canonical<'gcx, Ty<'tcx>>;

impl<'gcx: 'tcx, 'tcx> Canonicalize<'gcx, 'tcx> for Ty<'tcx> {
    type Canonicalized = CanonicalTy<'gcx>;

    fn intern(
        _gcx: TyCtxt<'_, 'gcx, 'gcx>,
        value: Canonical<'gcx, Self::Lifted>,
    ) -> Self::Canonicalized {
        value
    }
}
