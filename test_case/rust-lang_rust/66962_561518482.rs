rust
    crate fn eval_const_to_op(
        &self,
        val: &'tcx ty::Const<'tcx>,
        layout: Option<TyLayout<'tcx>>,
    ) -> InterpResult<'tcx, OpTy<'tcx, M::PointerTag>> {
        let tag_scalar = |scalar| match scalar {
            Scalar::Ptr(ptr) => Scalar::Ptr(self.tag_static_base_pointer(ptr)),
            Scalar::Raw { data, size } => Scalar::Raw { data, size },
        };
        // Early-return cases.
        let val_val = match val.val {
            ty::ConstKind::Param(_) =>
                throw_inval!(TooGeneric), // <--------------------
            ...
