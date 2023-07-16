
% cargo run
   Compiling rustc-serialize v0.3.16
   Compiling regex-syntax v0.2.2
   Compiling libc v0.2.2
   Compiling strsim v0.3.0
   Compiling memchr v0.1.7
   Compiling aho-corasick v0.4.0
   Compiling regex v0.1.43
   Compiling docopt v0.6.78
   Compiling docopt_macros v0.6.78
   Compiling 28089 v0.1.0 (file:///Users/lazarus/dev/tmp/bad-rust-plugin-2)
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'index out of bounds: the len is 60 but the index is 67', ../src/libcollections/vec.rs:1132

Could not compile `28089`.

To learn more, run the command again with --verbose.
% cargo version
cargo 0.8.0-nightly (dd32072 2015-12-12)
% rustc --version
rustc 1.7.0-nightly (110df043b 2015-12-13)
% uname -a
Darwin hog.local 14.5.0 Darwin Kernel Version 14.5.0: Tue Sep  1 21:23:09 PDT 2015; root:xnu-2782.50.1~1/RELEASE_X86_64 x86_64
