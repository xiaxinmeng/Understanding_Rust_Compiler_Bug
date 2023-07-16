rust
    /// In the case of closure types and fn pointers,
    /// we currently treat the input type parameters on the trait as
    /// outputs. This means that when we have a match we have only
    /// considered the self type, so we have to go back and make sure
    /// to relate the argument types too.  This is kind of wrong, but
    /// since we control the full set of impls, also not that wrong,
    /// and it DOES yield better error messages (since we don't report
    /// errors as if there is no applicable impl, but rather report
    /// errors are about mismatched argument types.
    ///
    /// Here is an example. Imagine we have a closure expression
    /// and we desugared it so that the type of the expression is
    /// `Closure`, and `Closure` expects an int as argument. Then it
    /// is "as if" the compiler generated this impl:
    ///
    ///     impl Fn(int) for Closure { ... }
    ///
    /// Now imagine our obligation is `Fn(usize) for Closure`. So far
    /// we have matched the self-type `Closure`. At this point we'll
    /// compare the `int` to `usize` and generate an error.
    ///
    /// Note that this checking occurs *after* the impl has selected,
    /// because these output type parameters should not affect the
    /// selection of the impl. Therefore, if there is a mismatch, we
    /// report an error to the user.
    fn confirm_poly_trait_refs(&mut self,
                               obligation_cause: ObligationCause<'tcx>,
                               obligation_param_env: ty::ParamEnv<'tcx>,
                               obligation_trait_ref: ty::PolyTraitRef<'tcx>,
                               expected_trait_ref: ty::PolyTraitRef<'tcx>)
                               -> Result<Vec<PredicateObligation<'tcx>>, SelectionError<'tcx>>
    {
        let obligation_trait_ref = obligation_trait_ref.clone();
        self.infcx
            .at(&obligation_cause, obligation_param_env)
            .sup(obligation_trait_ref, expected_trait_ref)
            .map(|InferOk { obligations, .. }| obligations)
            .map_err(|e| OutputTypeParameterMismatch(expected_trait_ref, obligation_trait_ref, e))
    }
