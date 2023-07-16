rust
assert_eq!(tcx.closure_base_def_id(self.mir_def_id), self.mir_def_id);
let identity_substs = Substs::identity_for_item(gcx, closure_base_def_id);
let substs =  self.infcx.replace_free_regions_with_nll_infer_vars(FR, &identity_substs);
DefiningTy::Const(self.mir_def_id, substs)
