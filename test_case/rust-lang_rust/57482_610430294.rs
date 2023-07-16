rust
// Build a tuple (U0..Un) of the final upvar types U0..Un
// and unify the upvar tupe type in the closure with it:
let final_upvar_tuple_type = tcx.mk_tuple(final_upvar_tys);
self.demand_suptype(span, substs.upvar_tuple_ty(), final_upvar_tuple_ty);
