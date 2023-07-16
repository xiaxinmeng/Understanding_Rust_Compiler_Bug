plain
   Compiling rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
   Compiling rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
   Compiling rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
   Compiling rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: internal compiler error: /checkout/compiler/rustc_const_eval/src/interpret/validity.rs:928:17: Unexpected error during validation: Fields cannot be extern types, unless they are at offset 0
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1170:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0-nightly (9c3d0345b 2021-12-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z symbol-mangling-version=v0 -Z macro-backtrace -Z tls-model=initial-exec -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `query_callbacks::upstream_drop_glue_for::force_from_dep_node`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
error: could not compile `rustc_query_impl`
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:08:56
