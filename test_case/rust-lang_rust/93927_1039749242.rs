rust
 if errors.len() == 1 {
    if let Some(pred) =
        errors[0].obligation.predicate.to_opt_poly_trait_pred()
    {
        self.infcx.suggest_restricting_param_bound(
            &mut err,
            pred,
            self.body_id,
        );
    } else if *ty != lhs_ty {
        // When we know that a missing bound is responsible, we don't show
        // this note as it is redundant.
        err.note(&format!(
            "the trait `{}` is not implemented for `{}`",
            missing_trait, lhs_ty
        ));
    }
}
