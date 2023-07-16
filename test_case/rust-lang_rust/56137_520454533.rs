rust
impl<'tcx> TypeFoldable<'tcx> for Constant<'tcx> {
    fn super_fold_with<F: TypeFolder<'tcx>>(&self, folder: &mut F) -> Self {
        Constant {
            span: self.span.clone(),
            ty: self.ty.fold_with(folder),
            user_ty: self.user_ty.fold_with(folder),
            literal: self.literal.fold_with(folder),
        }
    }
    fn super_visit_with<V: TypeVisitor<'tcx>>(&self, visitor: &mut V) -> bool {
        self.ty.visit_with(visitor) || self.literal.visit_with(visitor)
    }
}
