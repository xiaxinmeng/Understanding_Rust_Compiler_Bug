
warning: static is never used: `CALLBACKS`
 --> <source>:2:1
  |
2 | static CALLBACKS: Vec<dyn Fn(& _)> = Vec::new();
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: ErrorReported', compiler/rustc_mir/src/monomorphize/collector.rs:894:84
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.0-nightly (b7404c898 2021-09-03) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=1 -C llvm-args=--x86-asm-syntax=intel --crate-type rlib

query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
