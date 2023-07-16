plain
    Checking rustc-rayon v0.3.0
    Checking tempfile v3.1.0
    Checking regex v1.3.9
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error: unused import: `rustc_span::symbol::Ident`
  --> src/librustdoc/html/highlight.rs:15:5
15 | use rustc_span::symbol::Ident;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustdoc`

