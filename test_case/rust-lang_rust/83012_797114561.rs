
    Checking clippy_lints v0.1.52 (/checkout/src/tools/clippy/clippy_lints)
error: unnecessary parentheses around pattern
   --> src/tools/clippy/clippy_lints/src/loops/manual_memcpy.rs:206:13
    |
206 |         let (Sugg::NonParen(s) | Sugg::MaybeParen(s) | Sugg::BinOp(_, s)) = &self.0;
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses
    |
    = note: `-D unused-parens` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `clippy_lints`
