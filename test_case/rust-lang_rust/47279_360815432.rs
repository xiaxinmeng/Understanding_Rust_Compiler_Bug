rust
fn user_declared_mutable_variables(&self) -> impl Iterator<Item = Local> + 'cx {
    // the 'cx here is the lifetime of the `mir`

    self.mir.local_decls.iter_enumerated()
      .filter_map(|(local, local_decl)| {
           if local_decl.is_user_declared && /* is mutable */ { Some(local) } else { None }
       })
}
