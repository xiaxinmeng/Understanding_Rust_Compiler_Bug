
% rustc --version
rustc 1.16.0-nightly (47c8d9fdc 2017-01-08)
% rustc /tmp/issue-38942.rs --target x86_64-apple-darwin
% rustc /tmp/issue-38942.rs --target i686-apple-darwin
error: internal compiler error: /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/librustc_trans/mir/constant.rs:682: Unexpected non-fat-pointer operand

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

thread 'rustc' panicked at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/librustc_errors/lib.rs:423
note: Run with `RUST_BACKTRACE=1` for a backtrace.

% 
