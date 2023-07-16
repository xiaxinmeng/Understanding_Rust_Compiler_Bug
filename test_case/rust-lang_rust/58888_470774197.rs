
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:620:9
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
   1: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at src/libstd/sys_common/backtrace.rs:59
             at src/libstd/panicking.rs:197
   3: std::panicking::default_hook
             at src/libstd/panicking.rs:211
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:478
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::bug
   8: rustc::util::bug::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::util::bug::opt_span_bug_fmt
  13: rustc::util::bug::bug_fmt
  14: rustc_codegen_llvm::debuginfo::metadata::type_metadata
  15: rustc_codegen_llvm::debuginfo::<impl rustc_codegen_ssa::traits::debuginfo::DebugInfoBuilderMethods<'tcx> for rustc_codegen_llvm::builder::Builder<'a, 'll, 'tcx>>::declare_local
  16: core::iter::traits::iterator::Iterator::fold::{{closure}}
  17: <core::iter::adapters::Map<I, F> as core::iter::traits::iterator::Iterator>::fold
  18: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  19: rustc_codegen_ssa::mir::codegen_mir
  20: rustc_codegen_ssa::base::codegen_instance
  21: rustc_codegen_ssa::mono_item::MonoItemExt::define
  22: rustc_codegen_llvm::base::compile_codegen_unit::module_codegen
  23: rustc::dep_graph::graph::DepGraph::with_task
  24: rustc_codegen_llvm::base::compile_codegen_unit
  25: rustc_codegen_ssa::base::codegen_crate
  26: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  27: rustc::util::common::time
  28: rustc_driver::driver::phase_4_codegen
  29: <std::thread::local::LocalKey<T>>::with
  30: rustc::ty::context::TyCtxt::create_and_enter
  31: rustc_driver::driver::compile_input
  32: rustc_driver::run_compiler_with_pool
  33: <scoped_tls::ScopedKey<T>>::set
  34: rustc_driver::run_compiler
  35: syntax::with_globals
  36: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:87
  37: <F as alloc::boxed::FnBox<A>>::call_box
  38: std::sys::unix::thread::Thread::new::thread_start
             at /rustc/88f755f8a84df1d9e6b17cf10c96ae8b93481b2e/src/liballoc/boxed.rs:759
             at src/libstd/sys_common/thread.rs:14
             at src/libstd/sys/unix/thread.rs:80
  39: start_thread
  40: __clone
query stack during panic:
end of query stack
error: aborting due to previous error
