
warning: `extern` block uses type `[i8]` which is not FFI-safe: slices have no C equivalent
 --> src/main.rs:3:28
  |
3 |     pub static mut symbol: [c_char];
  |                            ^^^^^^^^
  |
  = note: #[warn(improper_ctypes)] on by default
  = help: consider using a raw pointer instead

thread 'main' panicked at 'assertion failed: !layout.is_unsized()', librustc_codegen_llvm/mir/place.rs:50:9
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:211
   3: std::panicking::default_hook
             at libstd/panicking.rs:227
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:481
   6: std::panicking::begin_panic
   7: rustc_codegen_llvm::mir::place::<impl rustc_codegen_llvm::mir::FunctionCx<'a, 'll, 'tcx>>::codegen_place
   8: rustc_codegen_llvm::mir::rvalue::<impl rustc_codegen_llvm::mir::FunctionCx<'a, 'll, 'tcx>>::codegen_rvalue_operand
   9: rustc_codegen_llvm::mir::rvalue::<impl rustc_codegen_llvm::mir::FunctionCx<'a, 'll, 'tcx>>::codegen_rvalue
  10: rustc_codegen_llvm::mir::codegen_mir
  11: rustc_codegen_llvm::base::codegen_instance
  12: rustc_codegen_llvm::mono_item::MonoItemExt::define
  13: rustc_codegen_llvm::base::compile_codegen_unit::module_codegen
  14: rustc::dep_graph::graph::DepGraph::with_task
  15: rustc_codegen_llvm::base::codegen_crate
  16: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  17: rustc::util::common::time
  18: rustc_driver::driver::phase_4_codegen
  19: rustc_driver::driver::compile_input::{{closure}}
  20: rustc::ty::context::tls::enter_context
  21: <std::thread::local::LocalKey<T>>::with
  22: rustc::ty::context::TyCtxt::create_and_enter
  23: rustc_driver::driver::compile_input
  24: rustc_driver::run_compiler_with_pool
  25: rustc_driver::driver::spawn_thread_pool
  26: rustc_driver::run_compiler
  27: <scoped_tls::ScopedKey<T>>::set
  28: syntax::with_globals
  29: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  30: rustc_driver::run
  31: rustc_driver::main
  32: std::rt::lang_start::{{closure}}
  33: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:310
  34: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  35: std::rt::lang_start_internal
             at libstd/panicking.rs:289
             at libstd/panic.rs:392
             at libstd/rt.rs:58
  36: main
  37: __libc_start_main
  38: <unknown>
query stack during panic:
end of query stack
