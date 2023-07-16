
$ cargo build --release --target armv7-unknown-linux-gnueabihf
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Size { raw: 604 }`,
 right: `Size { raw: 600 }`', src/librustc_codegen_llvm/type_of.rs:148:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: rustc 1.38.0-nightly (dddb7fca0 2019-07-30) running on x86_64-unknown-linux-gnu
note: compiler flags: -C opt-level=3 -C panic=abort -C linker=arm-linux-gnueabihf-gcc --crate-type bin
