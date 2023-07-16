plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0308]: mismatched types
   --> src/librustdoc/clean/types.rs:799:54
    |
799 |                         } else if mi.ident() == Some(sym::cfg) {
    |                                                      ^^^^^^^^ expected struct `rustc_span::symbol::Ident`, found struct `rustc_span::Symbol`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustdoc`
