
$ rustc +nightly foo.rs --crate-type cdylib -Copt-level=3 -C lto -C incremental=wut
$ rustc +nightly foo.rs --crate-type cdylib -Copt-level=3 -C lto -C incremental=wut
thread '<unnamed>' panicked at 'must have at least one serialized module', src/libcore/option.rs:1166:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
thread 'rustc' panicked at 'src/librustc_codegen_ssa/back/write.rs:1823: panic during codegen/LLVM phase', src/librustc/util/bug.rs:37:26

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.39.0-nightly (521d78407 2019-08-25) running on x86_64-unknown-linux-gnu

note: compiler flags: -C opt-level=3 -C lto -C incremental --crate-type cdylib
