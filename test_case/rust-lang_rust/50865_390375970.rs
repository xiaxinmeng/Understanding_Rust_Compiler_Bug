
error: internal compiler error: librustc_mir/monomorphize/collector.rs:750: Cannot create local mono-item for DefId(164/0:347 ~ gdbserver[d9d4]::sysroot_populator[0]::get_file_name_for_open_syscall[0])

thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:554:9
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
             at libstd/panicking.rs:467
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::bug
   8: rustc::session::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::session::opt_span_bug_fmt
  13: rustc::session::bug_fmt
  14: rustc_mir::monomorphize::collector::should_monomorphize_locally
  15: rustc_mir::monomorphize::collector::visit_instance_use
  16: <rustc_mir::monomorphize::collector::MirNeighborCollector<'a, 'tcx> as rustc::mir::visit::Visitor<'tcx>>::visit_terminator_kind
  17: rustc_mir::monomorphize::collector::collect_items_rec
  18: rustc_mir::monomorphize::collector::collect_items_rec
  19: rustc_mir::monomorphize::collector::collect_items_rec
  20: rustc_mir::monomorphize::collector::collect_items_rec
  21: rustc_mir::monomorphize::collector::collect_items_rec
  22: rustc_mir::monomorphize::collector::collect_items_rec
  23: rustc_mir::monomorphize::collector::collect_items_rec
  24: rustc_mir::monomorphize::collector::collect_items_rec
  25: rustc_mir::monomorphize::collector::collect_items_rec
  26: rustc_mir::monomorphize::collector::collect_items_rec
  27: rustc_mir::monomorphize::collector::collect_items_rec
  28: rustc_mir::monomorphize::collector::collect_items_rec
  29: rustc_mir::monomorphize::collector::collect_items_rec
  30: rustc_mir::monomorphize::collector::collect_items_rec
  31: rustc_mir::monomorphize::collector::collect_items_rec
  32: rustc_mir::monomorphize::collector::collect_items_rec
  33: rustc_mir::monomorphize::collector::collect_items_rec
  34: rustc_mir::monomorphize::collector::collect_items_rec
  35: rustc_mir::monomorphize::collector::collect_items_rec
  36: rustc_mir::monomorphize::collector::collect_items_rec
  37: rustc_mir::monomorphize::collector::collect_items_rec
  38: rustc_mir::monomorphize::collector::collect_items_rec
  39: rustc_mir::monomorphize::collector::collect_items_rec
  40: rustc_mir::monomorphize::collector::collect_items_rec
  41: rustc_mir::monomorphize::collector::collect_items_rec
  42: rustc_mir::monomorphize::collector::collect_items_rec
  43: rustc_mir::monomorphize::collector::collect_items_rec
  44: rustc_mir::monomorphize::collector::collect_items_rec
  45: rustc_mir::monomorphize::collector::collect_items_rec
  46: rustc_mir::monomorphize::collector::collect_items_rec
  47: rustc_mir::monomorphize::collector::collect_items_rec
  48: rustc_mir::monomorphize::collector::collect_items_rec
  49: rustc_mir::monomorphize::collector::collect_items_rec
  50: rustc_mir::monomorphize::collector::collect_items_rec
  51: rustc_mir::monomorphize::collector::collect_items_rec
  52: rustc_mir::monomorphize::collector::collect_items_rec
  53: rustc_mir::monomorphize::collector::collect_items_rec
  54: rustc_mir::monomorphize::collector::collect_items_rec
  55: rustc_mir::monomorphize::collector::collect_items_rec
  56: rustc_mir::monomorphize::collector::collect_items_rec
  57: rustc_mir::monomorphize::collector::collect_items_rec
  58: rustc_mir::monomorphize::collector::collect_items_rec
  59: rustc_mir::monomorphize::collector::collect_items_rec
  60: rustc_mir::monomorphize::collector::collect_items_rec
  61: rustc_mir::monomorphize::collector::collect_items_rec
  62: rustc_mir::monomorphize::collector::collect_items_rec
  63: rustc_mir::monomorphize::collector::collect_items_rec
  64: rustc_mir::monomorphize::collector::collect_items_rec
  65: rustc_mir::monomorphize::collector::collect_items_rec
  66: rustc_mir::monomorphize::collector::collect_crate_mono_items
  67: rustc::util::common::time
  68: rustc_codegen_llvm::base::collect_and_partition_mono_items
  69: rustc::ty::context::tls::with_context
  70: rustc::dep_graph::graph::DepGraph::with_task_impl
  71: rustc::ty::context::tls::with_related_context
  72: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  73: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  74: rustc_codegen_llvm::back::symbol_export::exported_symbols_provider_local
  75: rustc::ty::context::tls::with_context
  76: rustc::dep_graph::graph::DepGraph::with_task_impl
  77: rustc::ty::context::tls::with_related_context
  78: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  79: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  80: rustc_metadata::encoder::encode_metadata
  81: rustc_metadata::cstore_impl::<impl rustc::middle::cstore::CrateStore for rustc_metadata::cstore::CStore>::encode_metadata
  82: rustc::ty::context::TyCtxt::encode_metadata
  83: rustc_codegen_llvm::base::write_metadata
  84: rustc::util::common::time
  85: rustc_codegen_llvm::base::codegen_crate
  86: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  87: rustc::util::common::time
  88: rustc_driver::driver::phase_4_codegen
  89: rustc_driver::driver::compile_input::{{closure}}
  90: rustc::ty::context::tls::enter_context
  91: <std::thread::local::LocalKey<T>>::with
  92: rustc::ty::context::TyCtxt::create_and_enter
  93: rustc_driver::driver::compile_input
  94: rustc_driver::run_compiler_with_pool
  95: syntax::with_globals
  96: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  97: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:105
  98: rustc_driver::run
  99: rustc_driver::main
query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
#1 [exported_symbols] exported_symbols
end of query stack
