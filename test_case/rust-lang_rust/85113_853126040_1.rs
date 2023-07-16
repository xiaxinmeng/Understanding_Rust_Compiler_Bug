text
error: internal compiler error: broken MIR in DefId(0:8 ~ playground[2b78]::main) (NoSolution): failed to normalize `<_ as std::ops::FnOnce<()>>::Output`
  |
  = note: delayed at compiler/rustc_mir/src/borrow_check/type_check/mod.rs:252:27

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1021:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (625d5a693 2021-06-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `playground`

To learn more, run the command again with --verbose.
