error[E0658]: or-patterns syntax is experimental
   --> /Users/baner75418/.cargo/registry/src/github.com-1ecc6299db9ec823/grass-0.10.7/src/parse/value/css_function.rs:327:22
    |
327 |                 q @ ('"' | '\'') => {
    |                      ^^^^^^^^^^
    |
    = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information

error[E0658]: or-patterns syntax is experimental
   --> /Users/baner75418/.cargo/registry/src/github.com-1ecc6299db9ec823/grass-0.10.7/src/parse/value/css_function.rs:363:22
    |
363 |                 c @ (' ' | '\t') => {
    |                      ^^^^^^^^^^
    |
    = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information

error[E0658]: use of unstable library feature 'assoc_char_consts': recently added
  --> /Users/baner75418/.cargo/registry/src/github.com-1ecc6299db9ec823/grass-0.10.7/src/parse/value/css_function.rs:94:58
   |
94 |                 '!' | '%' | '&' | '*'..='~' | '\u{80}'..=char::MAX => buf.push(tok.kind),
   |                                                          ^^^^^^^^^
   |
   = note: see issue #71763 <https://github.com/rust-lang/rust/issues/71763> for more information

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0658`.
error: could not compile `grass`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...