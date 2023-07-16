rust
if self.is_mutable_static(def_id) {
    self.mk_mut_ref(self.lifetimes.re_erased, static_ty)
