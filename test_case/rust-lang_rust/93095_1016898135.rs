plain
    Checking askama v0.11.0
    Checking tracing-tree v0.2.0
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0615]: attempted to take value of method `ident` on type `&AssocItem`
    --> src/librustdoc/clean/types.rs:2225:72
     |
2225 |             .map(|did| tcx.provided_trait_methods(did).map(|meth| meth.ident.name).collect())
     |
help: use parentheses to call the method
     |
     |
2225 |             .map(|did| tcx.provided_trait_methods(did).map(|meth| meth.ident(_).name).collect())


error[E0615]: attempted to take value of method `ident` on type `&AssocItem`
   --> src/librustdoc/clean/mod.rs:390:60
    |
390 |             name: cx.tcx.associated_item(self.item_def_id).ident.name,
    |
help: use parentheses to call the method
    |
    |
390 |             name: cx.tcx.associated_item(self.item_def_id).ident(_).name,


error[E0615]: attempted to take value of method `ident` on type `&AssocItem`
    --> src/librustdoc/clean/mod.rs:1134:36
     |
1134 |                 let my_name = self.ident.name;
     |
help: use parentheses to call the method
     |
     |
1134 |                 let my_name = self.ident(_).name;


error[E0615]: attempted to take value of method `ident` on type `&AssocItem`
    --> src/librustdoc/clean/mod.rs:1200:60
     |
1200 |         Item::from_def_id_and_parts(self.def_id, Some(self.ident.name), kind, cx)
     |
help: use parentheses to call the method
     |
     |
1200 |         Item::from_def_id_and_parts(self.def_id, Some(self.ident(_).name), kind, cx)


error[E0615]: attempted to take value of method `ident` on type `&AssocItem`
    --> src/librustdoc/clean/mod.rs:1524:72
     |
1524 |                         name: cx.tcx.associated_item(pb.item_def_id()).ident.name,
     |
help: use parentheses to call the method
     |
     |
1524 |                         name: cx.tcx.associated_item(pb.item_def_id()).ident(_).name,


error[E0615]: attempted to take value of method `ident` on type `&AssocItem`
    --> src/librustdoc/clean/mod.rs:1595:50
1595 | ...                   .ident
     |                        ^^^^^ method, not a field
     |
help: use parentheses to call the method
help: use parentheses to call the method
     |
1595 |                                                 .ident(_)


error[E0615]: attempted to take value of method `ident` on type `&AssocItem`
    |
    |
431 |                     .any(|item| item.ident.name == variant_name)
    |
help: use parentheses to call the method
    |
    |
431 |                     .any(|item| item.ident(_).name == variant_name)

For more information about this error, try `rustc --explain E0615`.
error: could not compile `rustdoc` due to 7 previous errors
Build completed unsuccessfully in 0:02:44
