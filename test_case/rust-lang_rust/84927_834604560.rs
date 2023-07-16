plain
   Compiling rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
   Compiling rustc_lint v0.0.0 (/checkout/compiler/rustc_lint)
   Compiling rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
   Compiling rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
error: internal compiler error: compiler/rustc_codegen_llvm/src/context.rs:793:17: failed to get layout for `<Binders<chalk_ir::TraitRef<ChalkRustInterner>> as std::ptr::Pointee>::Metadata`: the type `<Binders<chalk_ir::TraitRef<ChalkRustInterner>> as std::ptr::Pointee>::Metadata` has an unknown layout

thread 'rustc' panicked at 'Box<Any>', /rustc/215738137bcbef2c3637a5bd290ef612cffe6ba5/library/std/src/panic.rs:59:5

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-beta.3 (215738137 2021-04-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z tls-model=initial-exec -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
