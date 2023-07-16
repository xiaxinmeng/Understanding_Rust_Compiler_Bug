plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0308]: mismatched types
   --> src/librustdoc/visit_ast.rs:288:50
    |
288 |                 self.check_item_attrs(item.span, def_id, &item.attrs, AttrReceiverKind::Use)
    |                                                  ^^^^^^ expected struct `LocalDefId`, found struct `rustc_span::def_id::DefId`
error[E0308]: mismatched types
   --> src/librustdoc/visit_ast.rs:291:46
    |
    |
291 |             self.check_item_attrs(item.span, def_id, &item.attrs, AttrReceiverKind::Other)
    |                                              ^^^^^^ expected struct `LocalDefId`, found struct `rustc_span::def_id::DefId`

error[E0599]: no method named `to_def_id` found for struct `rustc_span::def_id::DefId` in the current scope
   --> src/librustdoc/visit_ast.rs:297:36
    |
297 |             self.store_path(def_id.to_def_id());
    |                                    ^^^^^^^^^ method not found in `rustc_span::def_id::DefId`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0308, E0599.
For more information about an error, try `rustc --explain E0308`.
