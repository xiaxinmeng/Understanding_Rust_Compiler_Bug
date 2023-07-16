
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /rustc/0c987c5c02498b4e77f5dfae1f6914ffb9268575/src/libcore/macros/mod.rs:15:40
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:84
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:61
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1030
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1412
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:65
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:50
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:188
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:205
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:468
  12: std::panicking::continue_panic_fmt
             at src/libstd/panicking.rs:373
  13: rust_begin_unwind
             at src/libstd/panicking.rs:302
  14: core::panicking::panic_fmt
             at src/libcore/panicking.rs:82
  15: core::panicking::panic
             at src/libcore/panicking.rs:50
  16: <rustc_target::abi::call::FnAbi<&rustc::ty::TyS> as rustc::ty::layout::FnAbiExt<C>>::new
  17: <rustc_target::abi::TyLayout<&rustc::ty::TyS> as rustc_codegen_llvm::type_of::LayoutLlvmExt>::llvm_type
  18: rustc_codegen_ssa::mir::place::PlaceRef<V>::alloca
  19: rustc_codegen_ssa::mir::block::<impl rustc_codegen_ssa::mir::FunctionCx<Bx>>::codegen_terminator
  20: rustc_codegen_ssa::mir::codegen_mir
  21: rustc_codegen_ssa::base::codegen_instance
  22: <rustc::mir::mono::MonoItem as rustc_codegen_ssa::mono_item::MonoItemExt>::define
  23: rustc_codegen_llvm::base::compile_codegen_unit::module_codegen
  24: rustc::dep_graph::graph::DepGraph::with_task
  25: rustc_codegen_llvm::base::compile_codegen_unit
  26: rustc_codegen_ssa::base::codegen_crate
  27: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  28: rustc_interface::passes::start_codegen::{{closure}}
  29: rustc_interface::passes::start_codegen
  30: rustc::ty::context::tls::enter_global
  31: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  32: rustc_interface::passes::create_global_ctxt::{{closure}}
  33: rustc_interface::passes::BoxedGlobalCtxt::enter
  34: rustc_interface::queries::Query<T>::compute
  35: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::ongoing_codegen
  36: rustc_interface::interface::run_compiler_in_existing_thread_pool
  37: std::thread::local::LocalKey<T>::with
  38: scoped_tls::ScopedKey<T>::set
  39: syntax::with_globals
