plain
    Checking tracing-tree v0.2.0
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking tera v1.10.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0599]: no variant or associated item named `from_assoc_item` found for enum `UrlFragment` in the current scope
    |
    |
238 | crate enum UrlFragment {
    | ---------------------- variant or associated item `from_assoc_item` not found here
...
695 |                         let fragment = UrlFragment::from_assoc_item(item.def_id, kind, false);
    |                                                     ^^^^^^^^^^^^^^^ variant or associated item not found in `UrlFragment`
error[E0308]: mismatched types
   --> src/librustdoc/passes/collect_intra_doc_links.rs:693:21
    |
    |
693 | /                     assoc_item.map(|item| {
694 | |                         let kind = item.kind;
695 | |                         let fragment = UrlFragment::from_assoc_item(item.def_id, kind, false);
696 | |                         // HACK(jynelson): `clean` expects the type, not the associated item
...   |
699 | |                         (root_res, fragment, Some((kind.as_def_kind(), item.def_id)))
700 | |                     })
    | |______________________^ expected a tuple with 2 elements, found one with 3 elements
    |
    = note: expected enum `std::option::Option<(collect_intra_doc_links::Res, ItemFragment)>`
               found enum `std::option::Option<(collect_intra_doc_links::Res, _, std::option::Option<(DefKind, rustc_span::def_id::DefId)>)>`
Some errors have detailed explanations: E0308, E0599.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `rustdoc` due to 2 previous errors
Build completed unsuccessfully in 0:02:58
