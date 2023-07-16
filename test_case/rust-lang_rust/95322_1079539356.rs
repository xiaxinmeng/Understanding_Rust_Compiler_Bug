plain
    Checking clippy_lints v0.1.61 (/checkout/src/tools/clippy/clippy_lints)
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/regex.rs:165:56
    |
165 |                         str_span(expr.span, *e.span(), offset),
    |                                                        ^^^^^^ expected `u16`, found `u8`
    |
help: you can convert a `u8` to a `u16`
    |
165 |                         str_span(expr.span, *e.span(), offset.into()),

error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/regex.rs:173:56
    |
    |
173 |                         str_span(expr.span, *e.span(), offset),
    |                                                        ^^^^^^ expected `u16`, found `u8`
    |
help: you can convert a `u8` to a `u16`
    |
173 |                         str_span(expr.span, *e.span(), offset.into()),

For more information about this error, try `rustc --explain E0308`.
error: could not compile `clippy_lints` due to 2 previous errors
Build completed unsuccessfully in 0:03:40
