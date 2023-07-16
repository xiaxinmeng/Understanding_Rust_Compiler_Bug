
error[E0586]: inclusive range with no end
 --> src/test/ui/inline-const/const-match-pat-fail.rs:5:11
  |
5 |         1 ..= let 2 => {}
  |           ^^^ help: use `..` instead
  |
  = note: inclusive ranges must be bounded at the end (`..=b` or `a..=b`)

error: expected one of `=>`, `if`, or `|`, found keyword `let`
 --> src/test/ui/inline-const/const-match-pat-fail.rs:5:15
  |
5 |         1 ..= let 2 => {}
  |               ^^^ expected one of `=>`, `if`, or `|`

error[E0658]: half-open range patterns are unstable
 --> src/test/ui/inline-const/const-match-pat-fail.rs:5:9
  |
5 |         1 ..= let 2 => {}
  |         ^^^^^
  |
  = note: see issue #67264 <https://github.com/rust-lang/rust/issues/67264> for more information
  = help: add `#![feature(half_open_range_patterns)]` to the crate attributes to enable
