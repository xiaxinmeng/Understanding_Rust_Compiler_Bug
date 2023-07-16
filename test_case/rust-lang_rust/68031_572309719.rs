plain
2020-01-08T23:34:31.9457483Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2020-01-08T23:34:33.6038547Z    Compiling backtrace-sys v0.1.32
2020-01-08T23:34:33.6724638Z thread 'rustc' panicked at 'assertion failed: `(left == right)`
2020-01-08T23:34:33.6732133Z   left: `0`,
2020-01-08T23:34:33.6732674Z  right: `1`: destination and source slices have different lengths', /rustc/5193414545ba1db55e954d1a68f1943cf4f258df/src/libcore/macros/mod.rs:18:13
2020-01-08T23:34:33.6732867Z 
2020-01-08T23:34:33.6732962Z error: internal compiler error: unexpected panic
2020-01-08T23:34:33.6733005Z 
2020-01-08T23:34:33.6733242Z note: the compiler unexpectedly panicked. this is a bug.
2020-01-08T23:34:33.6733242Z note: the compiler unexpectedly panicked. this is a bug.
2020-01-08T23:34:33.6733284Z 
2020-01-08T23:34:33.6734120Z note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
2020-01-08T23:34:33.6734180Z 
2020-01-08T23:34:33.6734420Z note: rustc 1.42.0-nightly (519341454 2020-01-08) running on x86_64-unknown-linux-gnu
2020-01-08T23:34:33.6734672Z 
2020-01-08T23:34:33.6735660Z note: compiler flags: -Z external-macro-backtrace -Z save-analysis -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=2 -C codegen-units=1 -C debuginfo=1 -C linker=clang -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C debug-assertions=n --crate-type lib
2020-01-08T23:34:33.6735893Z note: some of the compiler flags provided by cargo are hidden
2020-01-08T23:34:33.6735966Z 
2020-01-08T23:34:33.7603833Z    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
2020-01-08T23:34:33.8232166Z    Compiling profiler_builtins v0.0.0 (/checkout/src/libprofiler_builtins)
---
2020-01-08T23:34:42.9178281Z   local time: Wed Jan  8 23:34:42 UTC 2020
2020-01-08T23:34:43.2460208Z   network time: Wed, 08 Jan 2020 23:34:43 GMT
2020-01-08T23:34:43.2461324Z == end clock drift check ==
2020-01-08T23:34:47.3871449Z 
2020-01-08T23:34:47.3989400Z ##[error]Bash exited with code '1'.
2020-01-08T23:34:47.4032290Z ##[section]Starting: Checkout
2020-01-08T23:34:47.4053823Z ==============================================================================
2020-01-08T23:34:47.4053930Z Task         : Get sources
2020-01-08T23:34:47.4054037Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
