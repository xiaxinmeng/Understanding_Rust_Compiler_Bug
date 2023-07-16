plain
   Compiling cc v1.0.60
   Compiling core v0.0.0 (/checkout/library/core)
   Compiling libc v0.2.85
   Compiling std v0.0.0 (/checkout/library/std)
thread 'rustc' panicked at 'non-eager expansion without a parent scope', compiler/rustc_resolve/src/macros.rs:238:22

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.50.0-beta.1 (05b602367 2020-12-29) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
