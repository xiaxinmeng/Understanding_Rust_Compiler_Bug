plain
   Compiling rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
   Compiling rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
   Compiling rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
   Compiling rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error: internal compiler error: compiler/rustc_codegen_llvm/src/context.rs:924:21: `fn_abi_of_instance(std::ptr::metadata::<chalk_ir::GenericArg<ChalkRustInterner>>, [])` failed: the type `<chalk_ir::GenericArg<ChalkRustInterner> as std::ptr::Pointee>::Metadata` has an unknown layout
  --> /checkout/library/core/src/ptr/metadata.rs:93:1
   |
93 | pub const fn metadata<T: ?Sized>(ptr: *const T) -> <T as Pointee>::Metadata {

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/28c8a34e18fc05277c81328d1bbf5ed931f4d22e/compiler/rustc_errors/src/lib.rs:1115:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0-beta.5 (28c8a34e1 2022-01-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z tls-model=initial-exec -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=v0 -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
