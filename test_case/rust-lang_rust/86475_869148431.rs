plain
   Compiling rustc_session v0.0.0 (/checkout/compiler/rustc_session)
   Compiling rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
   Compiling rustc_attr v0.0.0 (/checkout/compiler/rustc_attr)
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Mut`,
 right: `Not`', compiler/rustc_mir/src/const_eval/machine.rs:447:17

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (525c4c483 2021-06-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z tls-model=initial-exec -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [eval_to_allocation_raw] const-evaluating + checking `NO_ANN`
#1 [eval_to_const_value_raw] simplifying constant for the type system `NO_ANN`
error: could not compile `rustc_hir_pretty`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
