
error: internal compiler error: src/librustc/traits/codegen/mod.rs:55: Encountered error `Unimplemented` selecting `Binder(<() as std::ops::Deref>)` during codegen

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
  16: rustc::ty::context::GlobalCtxt::enter_local
  17: rustc::traits::codegen::codegen_fulfill_obligation
  18: rustc::ty::query::__query_compute::codegen_fulfill_obligation
  19: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::codegen_fulfill_obligation>::compute
  20: rustc::dep_graph::graph::DepGraph::with_task_impl
  21: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  22: rustc::ty::instance::Instance::resolve
  23: <rustc_mir::monomorphize::collector::MirNeighborCollector as rustc::mir::visit::Visitor>::visit_terminator_kind
  24: rustc_mir::monomorphize::collector::collect_items_rec
  25: rustc_mir::monomorphize::collector::collect_items_rec
  26: rustc_mir::monomorphize::collector::collect_crate_mono_items::{{closure}}
  27: rustc::util::common::time
  28: rustc_mir::monomorphize::collector::collect_crate_mono_items
  29: rustc::util::common::time
  30: rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items
  31: rustc::ty::query::__query_compute::collect_and_partition_mono_items
  32: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::collect_and_partition_mono_items>::compute
  33: rustc::dep_graph::graph::DepGraph::with_task_impl
  34: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  35: rustc_codegen_ssa::base::codegen_crate
  36: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  37: rustc::util::common::time
  38: rustc_interface::passes::start_codegen
  39: rustc::ty::context::tls::enter_global
  40: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  41: rustc_interface::passes::create_global_ctxt::{{closure}}
  42: rustc_interface::passes::BoxedGlobalCtxt::enter
  43: rustc_interface::queries::Query<T>::compute
  44: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::ongoing_codegen
  45: rustc_interface::interface::run_compiler_in_existing_thread_pool
  46: std::thread::local::LocalKey<T>::with
  47: scoped_tls::ScopedKey<T>::set
  48: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
query stack during panic:
#0 [codegen_fulfill_obligation] checking if `std::ops::Deref` fulfills its obligations
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: aborting due to previous error

