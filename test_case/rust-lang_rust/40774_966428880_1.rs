
if PrimTy::from_name(ident.name).is_none() {
    self.record_use(ident, binding, restricted_shadowing);
}
