plain
    Checking clippy_lints v0.1.62 (/checkout/src/tools/clippy/clippy_lints)
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/write.rs:457:72
    |
457 |                 if let Some(x) = self.named.iter_mut().find(|x| x.0 == n) {
    |                                                                        ^ expected struct `Symbol`, found `&str`
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/write.rs:465:38
    |
    |
465 |                     self.named.push((n, vec![span]));
    |                                      ^ expected struct `Symbol`, found `&str`
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/write.rs:467:38
    |
    |
467 |                     self.named.push((n, Vec::new()));
    |                                      ^ expected struct `Symbol`, found `&str`
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/write.rs:498:64
    |
    |
498 |                 .map_or(DUMMY_SP, |&x| str_lit.span.from_inner(x));
    |                                                                ^ expected struct `rustc_span::InnerSpan`, found struct `rustc_parse_format::InnerSpan`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `clippy_lints` due to 4 previous errors
Build completed unsuccessfully in 0:04:29
