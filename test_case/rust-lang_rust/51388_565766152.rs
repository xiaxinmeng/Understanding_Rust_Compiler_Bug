
error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:802: cannot create local mono-item for DefId(15:3 ~ bar[8787]::hello_world[0])

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:892:9
stack backtrace:
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: std::io::Write::write_fmt
   3: std::panicking::default_hook::{{closure}}
   4: std::panicking::default_hook
   5: rustc_driver::report_ice
   6: std::panicking::rust_panic_with_hook
   7: std::panicking::begin_panic
   8: rustc_errors::HandlerInner::bug
   9: rustc_errors::Handler::bug
  10: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  11: rustc::ty::context::tls::with_opt::{{closure}}
  12: rustc::ty::context::tls::with_context_opt
  13: rustc::ty::context::tls::with_opt
  14: rustc::util::bug::opt_span_bug_fmt
  15: rustc::util::bug::bug_fmt
  16: rustc_mir::monomorphize::collector::should_monomorphize_locally
  17: rustc_mir::monomorphize::collector::visit_instance_use
  18: <rustc_mir::monomorphize::collector::MirNeighborCollector as rustc::mir::visit::Visitor>::visit_terminator_kind
  19: rustc::mir::visit::Visitor::visit_body
  20: rustc_mir::monomorphize::collector::collect_items_rec
  21: rustc_mir::monomorphize::collector::collect_crate_mono_items::{{closure}}
  22: rustc::util::common::time
  23: rustc_mir::monomorphize::collector::collect_crate_mono_items
  24: rustc::util::common::time
  25: rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items
  26: rustc::ty::query::__query_compute::collect_and_partition_mono_items
  27: rustc::dep_graph::graph::DepGraph::with_task_impl
  28: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  29: rustc_codegen_ssa::back::symbol_export::exported_symbols_provider_local
  30: rustc::ty::query::__query_compute::exported_symbols
  31: rustc::dep_graph::graph::DepGraph::with_task_impl
  32: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  33: rustc_metadata::rmeta::encoder::EncodeContext::encode_crate_root
  34: rustc::ty::context::tls::with_context::{{closure}}
  35: rustc_metadata::rmeta::encoder::encode_metadata
  36: rustc_metadata::rmeta::decoder::cstore_impl::<impl rustc::middle::cstore::CrateStore for rustc_metadata::creader::CStore>::encode_metadata
  37: rustc::ty::context::TyCtxt::encode_metadata
  38: rustc_interface::passes::encode_and_write_metadata
  39: rustc::util::common::time
  40: rustc_interface::passes::start_codegen
  41: rustc::ty::context::tls::enter_global
  42: rustc_interface::queries::Query<T>::compute
  43: rustc_interface::queries::Queries::ongoing_codegen
  44: rustc_interface::interface::run_compiler_in_existing_thread_pool
  45: std::thread::local::LocalKey<T>::with
  46: scoped_tls::ScopedKey<T>::set
  47: syntax::with_globals
