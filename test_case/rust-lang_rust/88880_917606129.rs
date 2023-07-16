plain
    Checking clippy_lints v0.1.57 (/checkout/src/tools/clippy/clippy_lints)
error[E0425]: cannot find value `CRATE_DEF_ID` in this scope
   --> src/tools/clippy/clippy_lints/src/missing_doc.rs:107:66
    |
107 |         self.check_missing_docs_attrs(cx, attrs, cx.tcx.def_span(CRATE_DEF_ID), "the", "crate");
    |
help: consider importing one of these items
    |
8   | use rustc_hir::def_id::CRATE_DEF_ID;
