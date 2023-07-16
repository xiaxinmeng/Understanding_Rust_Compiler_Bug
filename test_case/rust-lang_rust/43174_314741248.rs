Rust
impl<'tcx> AsRef<LocalDecls<'tcx>> for LocalDecls<'tcx> { /* .. */ }
impl<'tcx> AsRef<LocalDecls<'tcx>> for Mir<'tcx> { /* .. */ }

    fn ty<'a, 'gcx, D: AsRef<LocalDecls>>(&self, local_decls: D, tcx: TyCtxt<'a, 'gcx, 'tcx>) -> LvalueTy<'tcx> {
        self.ty_(local_decls.as_ref(), tcx)
    }

    fn ty_<'a, 'gcx>(&self, local_decls: &LocalDecls<'tcx>, tcx: TyCtxt<'a, 'gcx, 'tcx>) -> LvalueTy<'tcx> {
        // ..
    }
