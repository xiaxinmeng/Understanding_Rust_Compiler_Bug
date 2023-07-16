plain
   Compiling miniz_oxide v0.4.0
   Compiling hashbrown v0.11.0
   Compiling object v0.22.0
   Compiling addr2line v0.14.0
error: internal compiler error: stable const functions must have either `rustc_const_stable` or `rustc_const_unstable` attribute
    |
    |
286 |     pub const fn is_zero(&self) -> bool {
    |
    |
    = note: delayed at compiler/rustc_mir/src/const_eval/fn_queries.rs:53:34

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1013:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-beta.3 (215738137 2021-04-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
