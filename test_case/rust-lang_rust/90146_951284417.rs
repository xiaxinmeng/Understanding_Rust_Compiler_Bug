plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0308]: mismatched types
  --> src/librustdoc/scrape_examples.rs:92:38
   |
92 |             tcx.hir().span_with_body(tcx.hir().get_parent_item(expr_id)).source_callsite();
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `HirId`, found struct `LocalDefId`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:02:55
