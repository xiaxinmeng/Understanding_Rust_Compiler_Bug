rust

impl<'tcx> QueryDescription<'tcx> for queries::type_op_prove_predicate<'tcx> {
    fn describe(_tcx: TyCtxt<'_, '_, '_>, goal: CanonicalTypeOpProvePredicateGoal<'tcx>)
                -> Cow<'static, str> {
        format!("evaluating `type_op_prove_predicate` `{:?}`", goal).into()
    }
}
