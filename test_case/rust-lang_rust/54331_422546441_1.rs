rust
let c_ty = self.fcx.inh.infcx.canonicalize_response(&ty);
self.fcx.tables.borrow_mut().user_provided_tys_mut().insert(t.hir_id, c_ty);
