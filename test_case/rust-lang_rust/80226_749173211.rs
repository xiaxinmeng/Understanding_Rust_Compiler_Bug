plain
    Checking rustc-rayon v0.3.0
    Checking tempfile v3.1.0
    Checking regex v1.3.9
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0599]: no method named `is_reserved_in_edition` found for struct `rustc_span::symbol::Ident` in the current scope
   --> src/librustdoc/html/highlight.rs:308:44
    |
308 |                 _ if Ident::from_str(text).is_reserved_in_edition(self.edition) => Class::KeyWord,
    |                                            ^^^^^^^^^^^^^^^^^^^^^^ method not found in `rustc_span::symbol::Ident`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustdoc`
