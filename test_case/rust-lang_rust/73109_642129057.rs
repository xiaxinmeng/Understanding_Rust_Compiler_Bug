
error: internal compiler error: src/librustc_middle/mir/tcx.rs:84: deref projection of non-dereferenceable ty PlaceTy { ty: usize, variant_index: None }

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:907:9
stack backtrace:
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: std::io::Write::write_fmt
   3: std::sys_common::backtrace::print
   4: std::panicking::default_hook::{{closure}}
   5: std::panicking::default_hook
   6: rustc_driver::report_ice
   7: std::panicking::rust_panic_with_hook
   8: std::panicking::begin_panic
   9: rustc_errors::HandlerInner::bug
  10: rustc_errors::Handler::bug
  11: rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}
  12: rustc_middle::ty::context::tls::with_opt::{{closure}}
  13: rustc_middle::ty::context::tls::with_opt
  14: rustc_middle::util::bug::opt_span_bug_fmt
  15: rustc_middle::util::bug::bug_fmt
  16: rustc_middle::mir::tcx::PlaceTy::projection_ty_core::{{closure}}
  17: rustc_middle::mir::tcx::PlaceTy::projection_ty_core
  18: rustc_middle::mir::tcx::PlaceTy::projection_ty
  19: <rustc_mir::monomorphize::collector::MirNeighborCollector as rustc_middle::mir::visit::Visitor>::visit_terminator_kind
  20: rustc_middle::mir::visit::Visitor::visit_body
  21: rustc_mir::monomorphize::collector::collect_neighbours
  22: rustc_data_structures::stack::ensure_sufficient_stack
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
  33: rustc_session::utils::<impl rustc_session::session::Session>::time
  34: rustc_mir::monomorphize::collector::collect_crate_mono_items
  35: rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items
  36: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::collect_and_partition_mono_items>::compute
  37: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  38: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  39: rustc_data_structures::stack::ensure_sufficient_stack
  40: rustc_query_system::query::plumbing::get_query_impl
  41: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
  42: rustc_session::utils::<impl rustc_session::session::Session>::time
  43: rustc_middle::ty::context::tls::enter_global
  44: rustc_interface::queries::Query<T>::compute
  45: rustc_interface::queries::Queries::ongoing_codegen
  46: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  47: rustc_span::with_source_map
  48: rustc_ast::attr::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
