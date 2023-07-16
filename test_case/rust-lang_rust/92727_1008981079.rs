plain
    Checking tracing-tree v0.2.0
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking tera v1.10.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0599]: no method named `maybe_collapsed_doc_value` found for struct `Rc<SharedContext<'_>>` in the current scope
    --> src/librustdoc/html/render/print_item.rs:1142:56
     |
1142 |                     if fields.iter().any(|f| cx.shared.maybe_collapsed_doc_value(f).is_some()) =>
     |                                                        ^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `Rc<SharedContext<'_>>`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:02:54
