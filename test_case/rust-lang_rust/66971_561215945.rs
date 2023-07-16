rust
    fn replace_with_const(
        &mut self,
        rval: &mut Rvalue<'tcx>,
        value: Const<'tcx>,
        source_info: SourceInfo,
    ) {
        ...
                Immediate::ScalarPair(
                    ScalarMaybeUndef::Scalar(one),
                    ScalarMaybeUndef::Scalar(two)
                ) => {
                    let ty = &value.layout.ty.kind;
                    if let ty::Tuple(substs) = ty {
                        debug!("substs: {:#?}", substs);
                        *rval = Rvalue::Aggregate(
                            Box::new(AggregateKind::Tuple),
                            vec![
                                self.operand_from_scalar(
                                    one, substs[0].expect_ty(), source_info.span
                                ),
                                self.operand_from_scalar(
                                    two, substs[1].expect_ty(), source_info.span
                                ),
                            ],
                        );
                    }
        ...
