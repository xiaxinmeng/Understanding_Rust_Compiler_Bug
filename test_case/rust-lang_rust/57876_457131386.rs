
warning: unused variable: `foo`
 --> src/main.rs:9:9
  |
9 |     let foo = unsafe { &FOO };
  |         ^^^ help: consider using `_foo` instead
  |
  = note: #[warn(unused_variables)] on by default

thread 'rustc' panicked at 'assertion failed: !layout.is_unsized()', src/librustc_codegen_ssa/mir/place.rs:35:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
   1: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:70
   2: std::panicking::default_hook::{{closure}}
             at src/libstd/sys_common/backtrace.rs:58
             at src/libstd/panicking.rs:200
   3: std::panicking::default_hook
             at src/libstd/panicking.rs:215
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:482
   6: std::panicking::begin_panic
   7: rustc_codegen_ssa::mir::place::<impl rustc_codegen_ssa::mir::FunctionCx<'a, 'tcx, Bx>>::codegen_place
   8: rustc_codegen_ssa::mir::rvalue::<impl rustc_codegen_ssa::mir::FunctionCx<'a, 'tcx, Bx>>::codegen_rvalue_operand
   9: rustc_codegen_ssa::mir::rvalue::<impl rustc_codegen_ssa::mir::FunctionCx<'a, 'tcx, Bx>>::codegen_rvalue
  10: rustc_codegen_ssa::mir::codegen_mir
  11: rustc_codegen_ssa::base::codegen_instance
  12: rustc_codegen_ssa::mono_item::MonoItemExt::define
  13: rustc_codegen_llvm::base::compile_codegen_unit::module_codegen
  14: rustc::dep_graph::graph::DepGraph::with_task
  15: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::ExtraBackendMethods>::compile_codegen_unit
  16: rustc_codegen_ssa::base::codegen_crate
  17: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  18: rustc::util::common::time
  19: rustc_driver::driver::phase_4_codegen
  20: rustc_driver::driver::compile_input::{{closure}}
  21: <std::thread::local::LocalKey<T>>::with
  22: rustc::ty::context::TyCtxt::create_and_enter
  23: rustc_driver::driver::compile_input
  24: <scoped_tls::ScopedKey<T>>::set
  25: rustc_driver::run_compiler
  26: <scoped_tls::ScopedKey<T>>::set
query stack during panic:
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.33.0-nightly (19f8958f8 2019-01-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `foo`.

To learn more, run the command again with --verbose.
