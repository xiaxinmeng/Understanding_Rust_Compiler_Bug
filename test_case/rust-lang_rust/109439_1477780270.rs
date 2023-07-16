plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0061]: this function takes 3 arguments but 4 arguments were supplied
    --> src/librustdoc/clean/mod.rs:2392:29
     |
2392 |         let (attrs, cfg) =  merge_attrs(cx, import_parent, &target_attrs, Some(&import_attrs));
     |                             ^^^^^^^^^^^     ------------- unexpected argument of type `std::option::Option<rustc_span::def_id::DefId>`
     |
note: expected `Option<(&[Attribute], Option<...>)>`, found `Option<&Vec<Attribute>>`
    --> src/librustdoc/clean/mod.rs:2392:75
     |
2392 |         let (attrs, cfg) =  merge_attrs(cx, import_parent, &target_attrs, Some(&import_attrs));
     |                                                                           ^^^^^^^^^^^^^^^^^^^
     = note: expected enum `std::option::Option<(&[Attribute], std::option::Option<rustc_span::def_id::DefId>)>`
                found enum `std::option::Option<&Vec<Attribute>>`
    --> src/librustdoc/clean/inline.rs:322:15
     |
322  | pub(crate) fn merge_attrs(
     |               ^^^^^^^^^^^
     |               ^^^^^^^^^^^
323  |     cx: &mut DocContext<'_>,
     |     -----------------------
324  |     old_attrs: &[ast::Attribute],
     |     ----------------------------
325  |     new_attrs: Option<(&[ast::Attribute], Option<DefId>)>,
help: remove the extra argument
     |
     |
2392 -         let (attrs, cfg) =  merge_attrs(cx, import_parent, &target_attrs, Some(&import_attrs));
2392 +         let (attrs, cfg) =  merge_attrs(cx, &target_attrs, /* std::option::Option<(&[Attribute], std::option::Option<rustc_span::def_id::DefId>)> */);

For more information about this error, try `rustc --explain E0061`.
[RUSTC-TIMING] rustdoc test:false 1.534
error: could not compile `rustdoc` due to previous error
