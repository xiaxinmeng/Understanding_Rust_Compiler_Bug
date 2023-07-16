plain
   Compiling rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
   Compiling rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
   Compiling rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
   Compiling rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_mir/src/monomorphize/collector.rs:968:22

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-beta.3 (215738137 2021-04-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z tls-model=initial-exec -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
error: could not compile `rustc_traits`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
