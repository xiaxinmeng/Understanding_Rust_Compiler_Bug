thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:518:9
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::span_bug
   8: rustc::session::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::session::opt_span_bug_fmt
  13: rustc::session::span_bug_fmt
  14: <rustc_mir::transform::generator::StateTransform as rustc_mir::transform::MirPass>::run_pass
  15: rustc_mir::transform::optimized_mir::{{closure}}
  16: rustc_mir::transform::optimized_mir
  17: rustc::ty::query::__query_compute::optimized_mir
  18: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::optimized_mir<'tcx>>::compute
  19: rustc::ty::context::tls::with_context
  20: rustc::dep_graph::graph::DepGraph::with_task_impl
  21: rustc::ty::context::tls::with_related_context
  22: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  23: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_get_query
  24: rustc::ty::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::instance_mir
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
  40: rustc_mir::monomorphize::collector::collect_crate_mono_items::{{closure}}
  41: rustc::util::common::time
  42: rustc_mir::monomorphize::collector::collect_crate_mono_items
  43: rustc::util::common::time
  44: rustc_codegen_llvm::base::collect_and_partition_mono_items
  45: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::collect_and_partition_mono_items<'tcx>>::compute
  46: rustc::ty::context::tls::with_context
  47: rustc::dep_graph::graph::DepGraph::with_task_impl
  48: rustc::ty::context::tls::with_related_context
  49: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  50: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  51: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  52: rustc::util::common::time
  53: rustc_driver::driver::phase_4_codegen
  54: rustc_driver::driver::compile_input::{{closure}}
  55: rustc::ty::context::tls::enter_context
  56: <std::thread::local::LocalKey<T>>::with
  57: rustc::ty::context::TyCtxt::create_and_enter
  58: rustc_driver::driver::compile_input
  59: rustc_driver::run_compiler_with_pool
  60: <scoped_tls::ScopedKey<T>>::set
  61: <scoped_tls::ScopedKey<T>>::set
  62: syntax::with_globals
  63: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  64: __rust_maybe_catch_panic
  65: rustc_driver::run
  66: rustc_driver::main
  67: std::rt::lang_start::{{closure}}
  68: std::panicking::try::do_call
  69: __rust_maybe_catch_panic
  70: std::rt::lang_start_internal
  71: main
  72: __libc_start_main
  73: _start
query stack during panic:
#0 [optimized_mir] processing `control_service::make_control_service::{{closure}}`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: aborting due to previous error

