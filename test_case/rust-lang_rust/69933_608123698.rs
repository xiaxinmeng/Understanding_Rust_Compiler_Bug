
RUSTFLAGS="-Z save-analysis" cargo check
   Compiling proc-macro2 v1.0.10
   Compiling unicode-xid v0.2.0
   Compiling syn v1.0.17
   Compiling quote v1.0.3
   Compiling async-attributes v1.1.1
error: internal compiler error: cannot lex `source_file` without source: /Users/gozala/.cargo/registry/src/github.com-1ecc6299db9ec823/quote-1.0.3/src/lib.rs

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:880:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.44.0-nightly (f509b26a7 2020-03-18) running on x86_64-apple-darwin

note: compiler flags: -Z save-analysis -C prefer-dynamic -C debuginfo=2 --crate-type proc-macro

note: some of the compiler flags provided by cargo are hidden

error: aborting due to previous error

error: could not compile `async-attributes`.

To learn more, run the command again with --verbose.
