plain
    Checking clippy_lints v0.1.55 (/checkout/src/tools/clippy/clippy_lints)
error[E0631]: type mismatch in closure arguments
   --> src/tools/clippy/clippy_lints/src/write.rs:655:87
    |
641 |     let mut cb = |r: Range<usize>, c: Result<char, EscapeError>| {
    |                  ----------------------------------------------- found signature of `fn(std::ops::Range<usize>, Result<char, EscapeError>) -> _`
...
655 |         StrStyle::Cooked => unescape::unescape_literal(contents, unescape::Mode::Str, &mut cb),
    |                                                                                       ^^^^^^^ expected signature of `fn(std::ops::Range<usize>, Result<Result<char, EscapeWarning>, EscapeError>) -> _`
   ::: /checkout/compiler/rustc_lexer/src/unescape.rs:74:8
    |
    |
74  |     F: FnMut(Range<usize>, Result<Result<char, EscapeWarning>, EscapeError>),
    |        --------------------------------------------------------------------- required by this bound in `unescape_literal`
error[E0631]: type mismatch in closure arguments
   --> src/tools/clippy/clippy_lints/src/write.rs:656:90
    |
    |
641 |     let mut cb = |r: Range<usize>, c: Result<char, EscapeError>| {
    |                  ----------------------------------------------- found signature of `fn(std::ops::Range<usize>, Result<char, EscapeError>) -> _`
...
656 |         StrStyle::Raw(_) => unescape::unescape_literal(contents, unescape::Mode::RawStr, &mut cb),
    |                                                                                          ^^^^^^^ expected signature of `fn(std::ops::Range<usize>, Result<Result<char, EscapeWarning>, EscapeError>) -> _`
   ::: /checkout/compiler/rustc_lexer/src/unescape.rs:74:8
    |
    |
74  |     F: FnMut(Range<usize>, Result<Result<char, EscapeWarning>, EscapeError>),
    |        --------------------------------------------------------------------- required by this bound in `unescape_literal`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0631`.
error: could not compile `clippy_lints`
