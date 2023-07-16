`
 warning: 11 warnings emitted

thread 'main' panicked at 'attempt to add with overflow', ../coverage/overflow.rs:10:18
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: unused `Result` that must be used
  --> ../coverage/closure_macro_async.rs:44:5
   |
44 |     executor::block_on(test());
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_must_use)]` on by default
   = note: this `Result` may be an `Err` variant, which should be handled

warning: 1 warning emitted

thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `try and succeed`,
 right: `try and succeed`: this assert should fail', ../coverage/issue-84561.rs:123:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Error: ()
warning: unused imports: `future::Future`, `marker::Send`, `pin::Pin`
 --> ../coverage/async2.rs:4:5
  |
4 |     future::Future,
  |     ^^^^^^^^^^^^^^
5 |     marker::Send,
  |     ^^^^^^^^^^^^
6 |     pin::Pin,
  |     ^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: 1 warning emitted

diff failed, and not suppressed without `// ignore-llvm-cov-show-diffs` in ../coverage/async.rs
make: *** [Makefile:123: async] Error 1
