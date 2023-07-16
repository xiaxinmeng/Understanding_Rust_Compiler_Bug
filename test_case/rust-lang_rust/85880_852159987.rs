plain
   Compiling unicode-normalization v0.1.13
   Compiling psm v0.1.11
   Compiling stacker v0.1.12
   Compiling rustc_llvm v0.0.0 (/checkout/compiler/rustc_llvm)
error: internal compiler error: tls access is checked in `Rvalue::ThreadLocalRef
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/tracing-core-0.1.17/src/field.rs:462:67
    |
462 |             fields: FieldSet::new(&[], crate::identify_callsite!(&NULL_CALLSITE)),
    |
    = note: delayed at compiler/rustc_mir/src/transform/check_consts/validation.rs:360:27


thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1021:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (88a1de446 2021-06-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z tls-model=initial-exec -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
