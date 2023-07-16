
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
  14: rustc::ty::context::GlobalCtxt::enter_local
  15: rustc::traits::codegen::codegen_fulfill_obligation
  16: rustc::ty::query::__query_compute::codegen_fulfill_obligation
  17: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::codegen_fulfill_obligation>::compute
  18: rustc::dep_graph::graph::DepGraph::with_task_impl
  19: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  20: rustc::ty::instance::Instance::resolve
  21: <rustc_mir::monomorphize::collector::RootCollector as rustc::hir::itemlikevisit::ItemLikeVisitor>::visit_item
  22: rustc::hir::Crate::visit_all_item_likes
  23: rustc_mir::monomorphize::collector::collect_roots
  24: rustc::util::common::time
  25: rustc_mir::monomorphize::collector::collect_crate_mono_items
  26: rustc::util::common::time
  27: rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items
  28: rustc::ty::query::__query_compute::collect_and_partition_mono_items
  29: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::collect_and_partition_mono_items>::compute
  30: rustc::dep_graph::graph::DepGraph::with_task_impl
  31: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  32: rustc_codegen_ssa::back::symbol_export::exported_symbols_provider_local
  33: rustc::ty::query::__query_compute::exported_symbols
  34: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::exported_symbols>::compute
  35: rustc::dep_graph::graph::DepGraph::with_task_impl
  36: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  37: rustc_metadata::encoder::encode_metadata
  38: rustc_metadata::cstore_impl::<impl rustc::middle::cstore::CrateStore for rustc_metadata::cstore::CStore>::encode_metadata
  39: rustc::ty::context::TyCtxt::encode_metadata
  40: rustc_codegen_llvm::base::write_metadata
  41: rustc::util::common::time
  42: rustc_codegen_ssa::base::codegen_crate
  43: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  44: rustc::util::common::time
  45: rustc_interface::passes::start_codegen
  46: rustc::ty::context::tls::enter_global
  47: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  48: rustc_interface::passes::create_global_ctxt::{{closure}}
  49: rustc_interface::passes::BoxedGlobalCtxt::enter
  50: rustc_interface::queries::Query<T>::compute
  51: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::ongoing_codegen
  52: rustc_interface::interface::run_compiler_in_existing_thread_pool
  53: std::thread::local::LocalKey<T>::with
  54: scoped_tls::ScopedKey<T>::set
  55: syntax::with_globals
query stack during panic:
#0 [codegen_fulfill_obligation] checking if `arbitrary::traits::Arbitrary` fulfills its obligations
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
#2 [exported_symbols] exported_symbols
end of query stack
