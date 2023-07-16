plain
    Checking rustc-rayon v0.3.0
    Checking regex v1.3.9
    Checking tempfile v3.1.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0599]: no method named `as_deref` found for enum `std::option::Option<rustc_span::Symbol>` in the current scope
    --> src/librustdoc/passes/collect_intra_doc_links.rs:1984:31
     |
1984 |     let item_name = item.name.as_deref().unwrap_or("<unknown>");
     |                               ^^^^^^^^ help: there is an associated function with a similar name: `as_ref`
    ::: /checkout/compiler/rustc_span/src/symbol.rs:1438:1
     |
     |
1438 | pub struct Symbol(SymbolIndex);
     | |
     | |
     | doesn't satisfy `<rustc_span::Symbol as __Deref>::Target = _`
     | doesn't satisfy `rustc_span::Symbol: __Deref`
     |
     = note: the method `as_deref` exists but the following trait bounds were not satisfied:
             `rustc_span::Symbol: __Deref`
             `<rustc_span::Symbol as __Deref>::Target = _`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustdoc`
