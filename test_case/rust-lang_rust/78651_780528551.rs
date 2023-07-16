
$ cat test.rs
use std::result;

impl result {
    fn into_future() -> Err {}
}

$ RUST_BACKTRACE=0 rustc +stage1 test.rs
error[E0573]: expected type, found module `result`
   --> test.rs:3:6
    |
3   | impl result {
    |      ^^^^^^ help: an enum with a similar name exists: `Result`
    |
   ::: /home/omer/rust/rust/library/core/src/result.rs:241:1
    |
241 | pub enum Result<T, E> {
    | --------------------- similarly named enum `Result` defined here

error[E0573]: expected type, found variant `Err`
 --> test.rs:4:25
  |
4 |     fn into_future() -> Err {}
  |                         ^^^ not a type
  |
thread 'rustc' panicked at 'assertion failed: !self.substitutions.is_empty()', compiler/rustc_errors/src/lib.rs:189:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-dev running on x86_64-unknown-linux-gnu

query stack during panic:
end of query stack
error: aborting due to previous error

For more information about this error, try `rustc --explain E0573`.
