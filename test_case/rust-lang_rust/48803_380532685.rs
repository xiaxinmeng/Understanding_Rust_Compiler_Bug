rust
self.prove_predicates(
  sig.inputs().iter().map(|ty| ty::PredicateWellFormed(ty))
);
