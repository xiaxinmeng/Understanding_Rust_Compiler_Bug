
# cargo install debcargo --verbose
    Updating registry `https://github.com/rust-lang/crates.io-index`
  Installing debcargo v1.3.0
error: failed to compile `debcargo v1.3.0`, intermediate artifacts can be found at `/tmp/cargo-install.euHYP8eHfK7K`

Caused by:
  failed to run `rustc` to learn about target-specific information

Caused by:
  process didn't exit successfully: `rustc - --crate-name ___ --print=file-names --crate-type bin --crate-type rlib --target x86_64-unknown-linux-gnu` (exit code: 101)
--- stderr
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

thread 'rustc' panicked at 'src/librustc/session/filesearch.rs:164: can't determine value for sysroot', src/librustc/session/mod.rs:839
note: Run with `RUST_BACKTRACE=1` for a backtrace.

