rust
/// Returns `true` if and only if this `op` should be const-propagated into.
fn should_const_prop(&mut self, op: OpTy<'tcx>) -> bool {
    let mir_opt_level = self.tcx.sess.opts.debugging_opts.mir_opt_level;

    if mir_opt_level == 0 {
        return false;
    }

    match *op {
        interpret::Operand::Immediate(Immediate::Scalar(ScalarMaybeUndef::Scalar(s))) => {
            s.is_bits()
        }
        interpret::Operand::Immediate(Immediate::ScalarPair(
            ScalarMaybeUndef::Scalar(l),
            ScalarMaybeUndef::Scalar(r),
        )) => l.is_bits() && r.is_bits(),
        interpret::Operand::Indirect(_) if mir_opt_level >= 2 => {
            let mplace = op.assert_mem_place(&self.ecx);
            intern_const_alloc_recursive(&mut self.ecx, InternKind::ConstProp, mplace, false)
                .expect("failed to intern alloc");
            true
        }
        _ => false,
    }
}
