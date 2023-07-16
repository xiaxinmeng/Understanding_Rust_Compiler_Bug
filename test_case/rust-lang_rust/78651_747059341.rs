
$ rustc --verbose --version
rustc 1.50.0-nightly (f76ecd066 2020-12-15)
binary: rustc
commit-hash: f76ecd0668fcdb289456cdc72a39ad15467cc454
commit-date: 2020-12-15
host: x86_64-unknown-linux-gnu
release: 1.50.0-nightly

$ rustc reduced-mutant.rs
error[E0573]: expected type, found module `result`
   --> reduced-mutant.rs:2:6
    |
2   |   impl result {
    |        ^^^^^^ help: an enum with a similar name exists: `Result`

error[E0573]: expected type, found variant `Err`
 --> reduced-mutant.rs:3:25
  |
3 |     fn into_future() -> Err {}
  |                         ^^^ not a type
  |
thread 'rustc' panicked at 'assertion failed: !self.substitutions.is_empty()', compiler/rustc_errors/src/lib.rs:151:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.50.0-nightly (f76ecd066 2020-12-15) running on x86_64-unknown-linux-gnu

query stack during panic:
end of query stack
error: aborting due to previous error

For more information about this error, try `rustc --explain E0573`.

$ cat reduced-mutant.rs
use std::result;
impl result {
    fn into_future() -> Err {}
}
$
