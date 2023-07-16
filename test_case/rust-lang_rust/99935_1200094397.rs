plain
   Doc-tests rustc_lint_defs


running 113 tests
ii..i..ii......i......i.ii..i.......i......iii..i...................i..........i..iF.... 88/113
failures:

---- src/builtin.rs - builtin::UNSTABLE_SYNTAX_PRE_EXPANSION (line 3212) stdout ----
---- src/builtin.rs - builtin::UNSTABLE_SYNTAX_PRE_EXPANSION (line 3212) stdout ----
error: expected `!` after `macro_rules`
  |
5 | macro_rules identity {
5 | macro_rules identity {
  | ^^^^^^^^^^^ help: add a `!`: `macro_rules!`

error: expected one of: `*`, `+`, or `?`
  |
  |
6 |    ( $($tokens:tt)* ) => { $($tokens) }

warning: unexpected `cfg` condition name
 --> src/builtin.rs:3219:7
  |
