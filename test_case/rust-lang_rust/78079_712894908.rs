
0: rust_begin_unwind
   1: std::panicking::begin_panic_fmt
   2: rustc_trait_selection::traits::codegen::codegen_fulfill_obligation
   3: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::codegen_fulfill_obligation>::compute
   4: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
   5: rustc_data_structures::stack::ensure_sufficient_stack
   6: rustc_query_system::query::plumbing::get_query_impl
   7: rustc_ty::instance::inner_resolve_instance
   8: rustc_ty::instance::resolve_instance
   9: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::resolve_instance>::compute
  10: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  11: rustc_data_structures::stack::ensure_sufficient_stack
  12: rustc_query_system::query::plumbing::get_query_impl
  13: rustc_middle::ty::instance::Instance::resolve_opt_const_arg
  14: rustc_middle::ty::instance::Instance::resolve
  15: rustc_mir_build::lints::Search::is_recursive_call
  16: rustc_data_structures::graph::iterate::TriColorDepthFirstSearch<G>::run_from_start
  17: rustc_mir_build::lints::check
  18: rustc_infer::infer::InferCtxtBuilder::enter
  19: rustc_mir_build::build::mir_built
  20: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_built>::compute
  21: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  22: rustc_data_structures::stack::ensure_sufficient_stack
  23: rustc_query_system::query::plumbing::get_query_impl
  24: rustc_mir::transform::check_unsafety::unsafety_check_result
  25: core::ops::function::FnOnce::call_once
  26: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::unsafety_check_result>::compute
  27: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  28: rustc_data_structures::stack::ensure_sufficient_stack
  29: rustc_query_system::query::plumbing::get_query_impl
  30: rustc_query_system::query::plumbing::ensure_query_impl
  31: rustc_mir::transform::mir_const
  32: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_const>::compute
  33: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  34: rustc_data_structures::stack::ensure_sufficient_stack
  35: rustc_query_system::query::plumbing::get_query_impl
  36: rustc_mir::transform::mir_promoted
  37: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_promoted>::compute
  38: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  39: rustc_data_structures::stack::ensure_sufficient_stack
  40: rustc_query_system::query::plumbing::get_query_impl
  41: rustc_mir::borrow_check::mir_borrowck
  42: core::ops::function::FnOnce::call_once
  43: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_borrowck>::compute
  44: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  45: rustc_data_structures::stack::ensure_sufficient_stack
  46: rustc_query_system::query::plumbing::get_query_impl
  47: rustc_query_system::query::plumbing::ensure_query_impl
  48: rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::par_body_owners
  49: rustc_interface::passes::analysis
  50: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
  51: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  52: rustc_data_structures::stack::ensure_sufficient_stack
  53: rustc_query_system::query::plumbing::get_query_impl
  54: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  55: rustc_span::with_source_map
  56: rustc_interface::interface::create_compiler_and_run
  57: scoped_tls::ScopedKey<T>::set
