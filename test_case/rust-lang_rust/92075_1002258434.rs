plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0308]: mismatched types
   --> src/librustdoc/passes/collect_intra_doc_links.rs:707:46
    |
707 |                     UrlFragment::StructField(field.ident),
    |                                              ^^^^^^^^^^^ expected struct `rustc_span::Symbol`, found struct `rustc_span::symbol::Ident`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:03:04
