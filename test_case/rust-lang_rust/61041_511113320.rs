
error: internal compiler error: src/librustc/ty/subst.rs:557: type parameter `Self/#0` (Self/0) out of range when substituting (root type=Some(<Self as float::Float>::Int)) substs=[]

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:584:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.38.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z external-macro-backtrace -Z always-encode-mir -Z mir-emit-retag -Z mir-opt-level=0 -Z force-unstable-if-unmarked -C opt-level=2 -C debuginfo=1 -C prefer-dynamic -C panic=abort -C debug-assertions=no -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `compiler_builtins`.
