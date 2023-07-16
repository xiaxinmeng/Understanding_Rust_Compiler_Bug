plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0425]: cannot find value `generic_args` in this scope
   --> src/librustdoc/clean/mod.rs:143:36
    |
143 |                 let generic_args = generic_args.clean(cx);

error[E0308]: mismatched types
   --> src/librustdoc/clean/mod.rs:139:46
    |
    |
139 |                     cx.tcx.require_lang_item(cx.tcx.lang_items().sized_trait(), Some(span));
    |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_hir::LangItem`, found enum `std::option::Option`
    |
    = note: expected enum `rustc_hir::LangItem`
               found enum `std::option::Option<rustc_span::def_id::DefId>`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0308, E0425.
For more information about an error, try `rustc --explain E0308`.
