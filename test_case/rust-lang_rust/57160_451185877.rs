
error[E0609]: no field `blacklisted_binding` on type `&mut Resolver<'a, 'crateloader>`resolve                                                                                                                                                                           
   --> src/librustc_resolve/resolve_imports.rs:242:57
    |
242 |                 if let Some(blacklisted_binding) = self.blacklisted_binding {
    |                                                         ^^^^^^^^^^^^^^^^^^^
