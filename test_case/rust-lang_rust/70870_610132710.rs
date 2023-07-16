
error: internal compiler error: src/librustc_typeck/variance/constraints.rs:354: unexpected type encountered in variance inference: FreshTy(0)

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:880:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.44.0-nightly (aa38c29f9 2020-04-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C codegen-units=1 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C debug-assertions=n --crate-type lib

note: some of the compiler flags provided by cargo are hidden
