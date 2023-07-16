diff
- let bound_vars = tcx
-             .mk_bound_variable_kinds(std::iter::once(ty::BoundVariableKind::Region(ty::BrAnon(0))));
+ let bound_vars = trait_ref.to_poly_trait_ref().bound_vars();
