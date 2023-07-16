
% rustc +nightly-2018-01-03 --crate-type lib issue-47309.rs  -C link-dead-code
% rustc +nightly-2018-01-04 --crate-type lib issue-47309.rs  -C link-dead-code
% rustc +nightly-2018-01-05 --crate-type lib issue-47309.rs  -C link-dead-code
error: internal compiler error: /checkout/src/librustc/ty/subst.rs:424: Region parameter out of range when substituting in region 'a (root type=None) (index=1)

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.24.0-nightly (8e7a609e6 2018-01-04) running on x86_64-unknown-linux-gnu

thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc_errors/lib.rs:451:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.

% rustc +nightly-2018-01-04 --version
rustc 1.24.0-nightly (0a3761e63 2018-01-03)
% rustc +nightly-2018-01-05 --version
rustc 1.24.0-nightly (8e7a609e6 2018-01-04)
%
