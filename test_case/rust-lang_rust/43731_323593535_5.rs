rust
    /// Applies the binary operation `op` to the two operands and writes a tuple of the result
    /// and a boolean signifying the potential overflow to the destination.
    pub fn binop_with_overflow(
        &mut self,
        op: mir::BinOp,
        left: ImmTy<'tcx, M::PointerTag>,
        right: ImmTy<'tcx, M::PointerTag>,
        dest: PlaceTy<'tcx, M::PointerTag>,
    ) -> EvalResult<'tcx> {
        let (val, overflowed) = self.binary_op_imm(op, left, right)?;
        let val = Immediate::ScalarPair(val.into(), Scalar::from_bool(overflowed).into());
        eprintln!("{:?}",val);                                                  
        self.write_immediate(val, dest)
    }
