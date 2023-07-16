plain
    Checking tracing-subscriber v0.2.16
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0599]: no variant or associated item named `BadPrefix` found for enum `rustc_lexer::TokenKind` in the current scope
   --> src/librustdoc/html/highlight.rs:416:46
    |
416 |             TokenKind::RawIdent | TokenKind::BadPrefix => Class::Ident,

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
