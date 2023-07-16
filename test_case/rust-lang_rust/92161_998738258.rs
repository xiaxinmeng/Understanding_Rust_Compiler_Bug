plain
    --> compiler/rustc_resolve/src/lib.rs:3422:28
     |
3422 |                   let attr = self
     |  ____________________________^
3423 | |                     .cstore()
3424 | |                     .item_attrs(def_id, self.session)
     | |_____________________________________________________^ creates a temporary which is freed while still in use
3425 |                       .iter()
3426 |                       .find(|a| a.has_name(sym::rustc_legacy_const_generics))?;
3427 |                   let mut ret = Vec::new();
3427 |                   let mut ret = Vec::new();
3428 |                   for meta in attr.meta_item_list()? {
     |
     = note: consider using a `let` binding to create a longer lived value

For more information about this error, try `rustc --explain E0716`.
