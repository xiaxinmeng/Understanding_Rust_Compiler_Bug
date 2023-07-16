rust
fn is_coinductive(&self, goal: &Canonical<'gcx, ty::ParamEnvAnd<'gcx, Goal<'gcx>>>) -> bool {
  let mut goal = &goal.value;
  loop {
    match goal {
      GoalKind::DomainGoal(DomainGoal::WellFormed(WellFormed::Trait(..))) => return true,
      GoalKind::DomainGoal(DomainGoal::Implemented(trait_predicate) => {
          return self.tcx.trait_is_auto(trait_predicate.def_id),
      }
      GoalKind::Quantified(_, bound_goal) => goal = bound_goal.skip_binder(),
      _ => return false,
    }
  }
}
