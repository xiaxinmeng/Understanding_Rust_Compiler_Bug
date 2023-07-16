plain
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_transmute v0.1.0 (/checkout/compiler/rustc_transmute)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0282]: type annotations needed for `Goal<'_, Q>`
    |
    |
551 |                 let trait_goal = goal.with(
    |
    |
help: consider giving `trait_goal` an explicit type, where the type for type parameter `Q` is specified
    |
551 |                 let trait_goal: Goal<'_, Q> = goal.with(


error[E0283]: type annotations needed for `Goal<'_, Q>`
    |
    |
551 |                 let trait_goal = goal.with(
    |                     ^^^^^^^^^^        ---- type must be known at this point
    |
    = note: multiple `impl`s satisfying `TraitPredicate<'_>: rustc_middle::ty::ToPredicate<'_, _>` found in the `rustc_middle` crate:
            - impl<'tcx, T> rustc_middle::ty::ToPredicate<'tcx, T> for T;
            - impl<'tcx> rustc_middle::ty::ToPredicate<'tcx, rustc_middle::ty::Binder<'tcx, TraitPredicate<'tcx>>> for TraitPredicate<'tcx>;
            - impl<'tcx> rustc_middle::ty::ToPredicate<'tcx> for TraitPredicate<'tcx>;
note: required by a bound in `Goal::<'tcx, P>::with`
    |
    |
37  |     pub fn with<Q>(self, tcx: TyCtxt<'tcx>, predicate: impl ToPredicate<'tcx, Q>) -> Goal<'tcx, Q> {
    |                                                             ^^^^^^^^^^^^^^^^^^^^ required by this bound in `Goal::<'tcx, P>::with`
help: consider giving `trait_goal` an explicit type, where the type for type parameter `Q` is specified
    |
551 |                 let trait_goal: Goal<'_, Q> = goal.with(

Some errors have detailed explanations: E0282, E0283.
For more information about an error, try `rustc --explain E0282`.
error: could not compile `rustc_trait_selection` (lib test) due to 2 previous errors
