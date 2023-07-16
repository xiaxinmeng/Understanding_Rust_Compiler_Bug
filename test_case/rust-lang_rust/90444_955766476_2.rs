
error: mismatched closing delimiter: `}`
 --> rustc_crash.rs:6:12
  |
5 | impl From<fn((), &[B], ())> for Thing {
  |                                       - closing delimiter possibly meant for this
6 |     fn from(_: fn((), &[B], ()) -> Self {}
  |            ^ unclosed delimiter
7 | }
  | ^ mismatched closing delimiter

error: expected one of `!`, `(`, `)`, `,`, `::`, or `<`, found `{`
 --> rustc_crash.rs:6:12
  |
6 |     fn from(_: fn((), &[B], ()) -> Self {}
  |            ^                           -^
  |            |                           |
  |            unclosed delimiter          help: `)` may belong here

error: non-item in item list
 --> rustc_crash.rs:7:1
  |
5 | impl From<fn((), &[B], ())> for Thing {
  |                                       - item list starts here
6 |     fn from(_: fn((), &[B], ()) -> Self {}
7 | }
  | ^
  | |
  | non-item starts here
  | item list ends here

error[E0601]: `main` function not found in crate `rustc_crash`
 --> rustc_crash.rs:1:1
  |
1 | / pub struct B(());
2 | |
3 | | enum Thing {}
4 | |
5 | | impl From<fn((), &[B], ())> for Thing {
6 | |     fn from(_: fn((), &[B], ()) -> Self {}
7 | | }
  | |_^ consider adding a `main` function to `rustc_crash.rs`

thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler\rustc_typeck\src\check\compare_method.rs:485:31
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.0 (09c42c458 2021-10-18) running on x86_64-pc-windows-msvc

query stack during panic:
#0 [check_mod_item_types] checking item types in top-level module
#1 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0601`.
