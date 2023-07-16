 console
$ cd bar

$ tail Cargo.toml
[dependencies]
foo = { path = "../foo" }

$ cargo build
   Compiling bar v0.1.0 (file:///home/japaric/tmp/bar)
error: internal compiler error: /checkout/src/librustc_metadata/cstore_impl.rs:131: get_optimized_mir: missing MIR for `DefId(2/0:5 ~ foo[128e]::start[0])`

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.24.0-nightly (77e189cd7 2017-12-28) running on x86_64-unknown-linux-gnu

thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc_errors/lib.rs:504:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: Could not compile `bar`.

To learn more, run the command again with --verbose.
