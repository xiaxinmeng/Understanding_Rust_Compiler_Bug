rust
    /// Returns `true` if and only if this `op` should be const-propagated into.
    fn should_const_prop(&mut self, op: OpTy<'tcx>) -> bool {
        let mir_opt_level = self.tcx.sess.opts.debugging_opts.mir_opt_level;

        if mir_opt_level == 0 {
            return false;
        }
