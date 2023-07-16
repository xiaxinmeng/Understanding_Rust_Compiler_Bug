rust
    pub(super) fn monomorphize<T: TypeFoldable<'tcx> + Subst<'tcx>>(
        &self,
        t: T,
    ) -> InterpResult<'tcx, T> {
        match self.stack.last() {
            Some(frame) => Ok(self.monomorphize_with_substs(t, frame.instance.substs)?),
            None => if t.needs_subst() {
                err!(TooGeneric).into()
            } else {
                Ok(t)
            },
        }
    }

    fn monomorphize_with_substs<T: TypeFoldable<'tcx> + Subst<'tcx>>(
        &self,
        t: T,
        substs: SubstsRef<'tcx>
    ) -> InterpResult<'tcx, T> {
        // miri doesn't care about lifetimes, and will choke on some crazy ones
        // let's simply get rid of them
        let substituted = t.subst(*self.tcx, substs);

        if substituted.needs_subst() {
            return err!(TooGeneric);
        }

        Ok(self.tcx.normalize_erasing_regions(ty::ParamEnv::reveal_all(), substituted))
    }
