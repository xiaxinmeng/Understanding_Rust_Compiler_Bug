rust
adt_def.non_enum_variant().fields.iter().any(|field| {
    self.tcx.type_of(field.did).needs_drop(self.tcx, param_env)
})
