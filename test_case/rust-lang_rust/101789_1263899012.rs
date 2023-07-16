plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
error[E0308]: mismatched types
    --> compiler/rustc_parse/src/parser/item.rs:1792:74
     |
1792 |                 if let Some((ident, _)) = self.token.ident() && ident == "let" {
     |                                                                          ^^^^^ expected struct `rustc_span::symbol::Ident`, found `&str`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_parse` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_parse` due to previous error
