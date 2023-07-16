plain
    Checking rustc-rayon v0.3.0
    Checking regex v1.3.9
    Checking tempfile v3.1.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0599]: no method named `lines` found for struct `rustc_span::Symbol` in the current scope
   --> src/librustdoc/clean/types.rs:628:35
    |
628 |                 doc_line += value.lines().count();
    |                                   ^^^^^ method not found in `rustc_span::Symbol`
error[E0308]: mismatched types
   --> src/librustdoc/clean/types.rs:632:26
    |
632 |                     doc: value,
632 |                     doc: value,
    |                          ^^^^^ expected struct `std::string::String`, found struct `rustc_span::Symbol`
help: try using a conversion method
    |
    |
632 |                     doc: value.to_string(),
    |                          ^^^^^^^^^^^^^^^^^
632 |                     doc: value.to_string(),

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0308, E0599.
