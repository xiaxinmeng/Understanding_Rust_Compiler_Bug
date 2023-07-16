rust
impl<'tcx> TypeFoldable<'tcx> for &'tcx ty::Const<'tcx> {
    fn super_fold_with<F: TypeFolder<'tcx>>(&self, folder: &mut F) -> Self {
        let ty = self.ty.fold_with(folder);
        let val = self.val.fold_with(folder);
        folder.tcx().mk_const(ty::Const {
            ty,
            val
        })
    }

    fn fold_with<F: TypeFolder<'tcx>>(&self, folder: &mut F) -> Self {
        folder.fold_const(*self)
    }

    fn super_visit_with<V: TypeVisitor<'tcx>>(&self, visitor: &mut V) -> bool {
        self.ty.visit_with(visitor) || self.val.visit_with(visitor)
    }

    fn visit_with<V: TypeVisitor<'tcx>>(&self, visitor: &mut V) -> bool {
        visitor.visit_const(self)
    }
}
