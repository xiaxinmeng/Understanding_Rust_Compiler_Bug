
if let Err(FixupError::UnresolvedTy(x)) = self.fcx.fully_resolve(&ty) {
   let ty = self.fcx.probe_ty_var(x);
