
thread 'rustc' panicked at 'assertion failed: mir.arg_count == 0', src/librustc_mir/const_eval.rs:148:5                                                                     
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::begin_panic
   7: rustc_mir::const_eval::eval_body_using_ecx
   8: rustc_mir::const_eval::const_eval_raw_provider
   9: rustc::ty::query::__query_compute::const_eval_raw
  10: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::const_eval_raw<'tcx>>::compute
  11: rustc::dep_graph::graph::DepGraph::with_task_impl
  12: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  13: rustc_mir::const_eval::const_eval_provider
  14: rustc::ty::query::__query_compute::const_eval
  15: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::const_eval<'tcx>>::compute
  16: rustc::dep_graph::graph::DepGraph::with_task_impl
  17: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  18: rustc_mir::const_eval::const_eval_provider
  19: rustc::ty::query::__query_compute::const_eval
  20: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::const_eval<'tcx>>::compute
  21: rustc::dep_graph::graph::DepGraph::with_task_impl
  22: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  23: rustc_mir::monomorphize::collector::collect_items_rec
  24: rustc_mir::monomorphize::collector::collect_items_rec
  25: rustc_mir::monomorphize::collector::collect_items_rec
  26: rustc_mir::monomorphize::collector::collect_items_rec
  27: rustc_mir::monomorphize::collector::collect_items_rec
  28: rustc_mir::monomorphize::collector::collect_items_rec
  29: rustc_mir::monomorphize::collector::collect_items_rec
  30: rustc_mir::monomorphize::collector::collect_crate_mono_items::{{closure}}
  31: rustc::util::common::time
  32: rustc_mir::monomorphize::collector::collect_crate_mono_items
  33: rustc::util::common::time
  34: rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items
  35: rustc::ty::query::__query_compute::collect_and_partition_mono_items
  36: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::collect_and_partition_mono_items<'tcx>>::compute
  37: rustc::dep_graph::graph::DepGraph::with_task_impl
  38: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  39: rustc_codegen_ssa::base::codegen_crate
  40: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  41: rustc::util::common::time
  42: rustc_interface::passes::start_codegen
  43: rustc::ty::context::tls::enter_global
  44: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  45: rustc_interface::passes::create_global_ctxt::{{closure}}
  46: rustc_interface::passes::BoxedGlobalCtxt::enter
  47: <rustc_interface::queries::Query<T>>::compute
  48: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::ongoing_codegen
  49: rustc_interface::interface::run_compiler_in_existing_thread_pool
  50: <std::thread::local::LocalKey<T>>::with
  51: <scoped_tls::ScopedKey<T>>::set
  52: syntax::with_globals
query stack during panic:
#0 [const_eval_raw] const-evaluating `slice::slice_index_order_fail`
#1 [const_eval] const-evaluating + checking `slice::slice_index_order_fail`
#2 [const_eval] const-evaluating + checking `slice::slice_index_order_fail`
#3 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
