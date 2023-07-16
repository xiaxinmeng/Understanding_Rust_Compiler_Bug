rust
    fn confirm_fn_pointer_candidate(&mut self, obligation: &TraitObligation<'tcx>)
        -> Result<VtableFnPointerData<'tcx, PredicateObligation<'tcx>>, SelectionError<'tcx>>
    {
        debug!("confirm_fn_pointer_candidate({:?})",
               obligation);

        // ok to skip binder; it is reintroduced below
        let self_ty = self.infcx.shallow_resolve(*obligation.self_ty().skip_binder());
        let sig = self_ty.fn_sig(self.tcx());
        let trait_ref =
            self.tcx().closure_trait_ref_and_return_type(obligation.predicate.def_id(),
                                                         self_ty,
                                                         sig,
                                                         util::TupleArgumentsFlag::Yes)
            .map_bound(|(trait_ref, _)| trait_ref);

        let Normalized { value: trait_ref, obligations } =
            project::normalize_with_depth(self,
                                          obligation.param_env,
                                          obligation.cause.clone(),
                                          obligation.recursion_depth + 1,
                                          &trait_ref);

        self.confirm_poly_trait_refs(obligation.cause.clone(),
                                     obligation.param_env,
                                     obligation.predicate.to_poly_trait_ref(),
                                     trait_ref)?;
        Ok(VtableFnPointerData { fn_ty: self_ty, nested: obligations })
    }
