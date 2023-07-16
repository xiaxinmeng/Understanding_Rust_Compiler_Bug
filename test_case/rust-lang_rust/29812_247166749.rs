
$ rustup run stable rustc -V
rustc 1.11.0 (9b21dcd6a 2016-08-15)

$ rustup run stable rustc --target aarch64-apple-ios --print cfg
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'failed to get iphoneos SDK path: No such file or directory (os error 2)', ../src/librustc_back/target/apple_ios_base.rs:59
