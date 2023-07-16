rust
impl<'a, 'gcx, 'tcx> Deref for TyCtxt<'a, 'gcx, 'tcx> {
    type Target = &'a GlobalCtxt<'gcx>;
    fn deref(&self) -> &Self::Target {
        &self.gcx
    }
}
