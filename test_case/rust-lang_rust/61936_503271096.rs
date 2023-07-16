
error: internal compiler error: src/librustc_codegen_llvm/context.rs:856: failed to get layout for `[i32; _]`: the type `[i32; _]` has an unknown layout

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:643:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:47
   3: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:36
   4: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:198
   5: std::panicking::default_hook
             at src/libstd/panicking.rs:212
   6: rustc::util::common::panic_hook
   7: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:479
   8: std::panicking::begin_panic
   9: rustc_errors::Handler::bug
  10: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  11: rustc::ty::context::tls::with_opt::{{closure}}
  12: rustc::ty::context::tls::with_context_opt
  13: rustc::ty::context::tls::with_opt
  14: rustc::util::bug::opt_span_bug_fmt
  15: rustc::util::bug::bug_fmt
  16: <rustc_codegen_llvm::context::CodegenCx as rustc_target::abi::LayoutOf>::layout_of::{{closure}}
  17: <rustc_codegen_llvm::context::CodegenCx as rustc_target::abi::LayoutOf>::layout_of
  18: <rustc_target::abi::call::FnType<&rustc::ty::TyS> as rustc::ty::layout::FnTypeExt<C>>::new
  19: rustc_codegen_llvm::declare::<impl rustc_codegen_ssa::traits::declare::DeclareMethods for rustc_codegen_llvm::context::CodegenCx>::declare_fn
  20: rustc_codegen_llvm::mono_item::<impl rustc_codegen_ssa::traits::declare::PreDefineMethods for rustc_codegen_llvm::context::CodegenCx>::predefine_fn
  21: <rustc::mir::mono::MonoItem as rustc_codegen_ssa::mono_item::MonoItemExt>::predefine
  22: rustc_codegen_llvm::base::compile_codegen_unit::module_codegen
  23: rustc::dep_graph::graph::DepGraph::with_task
  24: rustc_codegen_llvm::base::compile_codegen_unit
  25: rustc_codegen_ssa::base::codegen_crate
  26: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  27: rustc::util::common::time
  28: rustc_interface::passes::start_codegen
  29: rustc::ty::context::tls::enter_global
  30: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  31: rustc_interface::passes::create_global_ctxt::{{closure}}
  32: rustc_interface::passes::BoxedGlobalCtxt::enter
  33: rustc_interface::queries::Query<T>::compute
  34: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::ongoing_codegen
  35: rustc_interface::interface::run_compiler_in_existing_thread_pool
  36: std::thread::local::LocalKey<T>::with
  37: scoped_tls::ScopedKey<T>::set
  38: syntax::with_globals
