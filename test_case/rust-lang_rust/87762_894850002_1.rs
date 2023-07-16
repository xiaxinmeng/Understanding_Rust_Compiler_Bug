
   Compiling playground v0.0.1 (/playground)
error: internal compiler error: broken MIR in DefId(0:14 ~ playground[4e8d]::main::{closure#0}) (bb0[0]): equate_normalized_input_or_output: `&[closure@src/main.rs:17:13: 17:19]==&[closure@src/main.rs:17:13: 17:19]` failed with `NoSolution`
  --> src/main.rs:17:19
   |
17 |     f(Type, |_| ());
   |                   ^
   |
   = note: delayed at compiler/rustc_mir/src/borrow_check/type_check/mod.rs:299:27

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1065:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.0-nightly (574d37568 2021-08-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `playground`
