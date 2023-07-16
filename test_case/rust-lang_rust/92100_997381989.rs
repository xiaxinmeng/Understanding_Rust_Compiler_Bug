`
   Compiling v v0.1.0 (/tmp/v)
error[E0425]: cannot find value `a` in this scope
 --> src/main.rs:7:10
  |
7 |         [a..mid, mid, mid..b] => {}
  |          ^ help: a local variable with a similar name exists: `s`

error[E0425]: cannot find value `b` in this scope
 --> src/main.rs:7:28
  |
7 |         [a..mid, mid, mid..b] => {}
  |                            ^ help: a local variable with a similar name exists: `s`

error[E0658]: exclusive range pattern syntax is experimental
 --> src/main.rs:7:10
  |
7 |         [a..mid, mid, mid..b] => {}
  |          ^^^^^^
  |
  = note: see issue #37854 <https://github.com/rust-lang/rust/issues/37854> for more information
  = help: add `#![feature(exclusive_range_pattern)]` to the crate attributes to enable

error[E0658]: exclusive range pattern syntax is experimental
 --> src/main.rs:7:23
  |
7 |         [a..mid, mid, mid..b] => {}
  |                       ^^^^^^
  |
  = note: see issue #37854 <https://github.com/rust-lang/rust/issues/37854> for more information
  = help: add `#![feature(exclusive_range_pattern)]` to the crate attributes to enable

thread 'rustc' panicked at 'expected `NodeId` to be lowered already for res Local(
    NodeId(40),
)', compiler/rustc_ast_lowering/src/lib.rs:563:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0-nightly (91a0600a5 2021-12-18) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
Some errors have detailed explanations: E0425, E0658.
For more information about an error, try `rustc --explain E0425`.
error: could not compile `v` due to 4 previous errors
