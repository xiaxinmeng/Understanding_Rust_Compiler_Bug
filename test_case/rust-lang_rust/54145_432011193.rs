plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:148a6ba0
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:55:27] warning:   ^
[00:55:27] warning: 2 warnings generated.
[00:55:27]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:55:27]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:55:57] error: internal compiler error: librustc/hir/map/mod.rs:520: couldn't find node id 4294967295 in the AST map
[00:55:57] thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:600:9
[00:55:57] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:55:58] error: aborting due to previous error
[00:55:58] 
[00:55:58] 
[00:55:58] 
[00:55:58] note: the compiler unexpectedly panicked. this is a bug.
[00:55:58] 
[00:55:58] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:55:58] 
[00:55:58] note: rustc 1.31.0-nightly (2d61cfc90 2018-10-22) running on x86_64-unknown-linux-gnu
[00:55:58] 
[00:55:58] note: compiler flags: -Z save-analysis -Z force-unstable-if-unmarked -C opt-level=2 -C linker=clang -C prefer-dynamic -C linker=clang -C debuginfo=1 -C debug-assertions=n -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:55:58] note: some of the compiler flags provided by cargo are hidden
[00:55:58] 
[00:55:58] error: Could not compile `core`.
[00:55:58] warning: build failed, waiting for other jobs to finish...
