
thread 'rustc' panicked at '`SaveContext::typeck_results` called outside of body', compiler/rustc_save_analysis/src/lib.rs:522:23
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (65f3f8b22 2021-11-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z save-analysis -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
