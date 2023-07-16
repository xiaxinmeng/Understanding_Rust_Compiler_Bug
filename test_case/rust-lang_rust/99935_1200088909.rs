plain

   Doc-tests rustc_lint_defs

running 113 tests
ii..i..ii......i.......iii..i.......i......iii..i...................i..........i..iF.... 88/113
failures:

---- src/builtin.rs - builtin::UNSTABLE_SYNTAX_PRE_EXPANSION (line 3212) stdout ----
---- src/builtin.rs - builtin::UNSTABLE_SYNTAX_PRE_EXPANSION (line 3212) stdout ----
error: expected `!` after `macro_rules`
  |
5 | macro_rules identity {
5 | macro_rules identity {
  | ^^^^^^^^^^^ help: add a `!`: `macro_rules!`

error[E0658]: the `#[expect]` attribute is an experimental feature
   |
   |
14 | #[expect(unstable_syntax_pre_expansion)]
   |
   = note: see issue #54503 <https://github.com/rust-lang/rust/issues/54503> for more information
   = help: add `#![feature(lint_reasons)]` to the crate attributes to enable


error: expected one of: `*`, `+`, or `?`
  |
  |
6 |    ( $($tokens:tt)* ) => { $($tokens) }

warning: unexpected `cfg` condition name
 --> src/builtin.rs:3219:7
  |
