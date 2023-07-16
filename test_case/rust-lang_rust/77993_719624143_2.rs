
stack backtrace:
   0: std::panicking::begin_panic
   1: rustc_errors::HandlerInner::bug
   2: rustc_errors::Handler::bug
   3: rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}
   4: rustc_middle::ty::context::tls::with_opt::{{closure}}
   5: rustc_middle::ty::context::tls::with_opt
   6: rustc_middle::util::bug::opt_span_bug_fmt
   7: rustc_middle::util::bug::bug_fmt
   8: rustc_middle::ty::sty::<impl rustc_middle::ty::TyS>::tuple_fields
   9: rustc_trait_selection::traits::select::SelectionContext::copy_clone_conditions
  10: rustc_trait_selection::traits::select::candidate_assembly::<impl rustc_trait_selection::traits::select::SelectionContext>::assemble_candidates
  11: rustc_trait_selection::traits::select::candidate_assembly::<impl rustc_trait_selection::traits::select::SelectionContext>::candidate_from_obligation_no_cache
  12: rustc_query_system::dep_graph::graph::DepGraph<K>::with_anon_task
  13: rustc_trait_selection::traits::select::candidate_assembly::<impl rustc_trait_selection::traits::select::SelectionContext>::candidate_from_obligation
  14: rustc_trait_selection::traits::select::SelectionContext::select
  15: rustc_trait_selection::traits::fulfill::FulfillProcessor::process_trait_obligation
  16: rustc_trait_selection::traits::fulfill::FulfillProcessor::progress_changed_obligations
  17: rustc_data_structures::obligation_forest::ObligationForest<O>::process_obligations
  18: <rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::select_where_possible
  19: rustc_typeck::check::fn_ctxt::_impl::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::select_obligations_where_possible
  20: rustc_infer::infer::InferCtxtBuilder::enter
  21: rustc_typeck::check::typeck
  22: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::typeck>::compute
  23: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  24: rustc_data_structures::stack::ensure_sufficient_stack
  25: rustc_query_system::query::plumbing::get_query_impl
  26: rustc_query_system::query::plumbing::ensure_query_impl
  27: rustc_typeck::check::typeck_item_bodies
  28: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::typeck_item_bodies>::compute
  29: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  30: rustc_data_structures::stack::ensure_sufficient_stack
  31: rustc_query_system::query::plumbing::get_query_impl
  32: rustc_typeck::check_crate
  33: rustc_interface::passes::analysis
  34: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
  35: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  36: rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}
  37: rustc_query_system::query::plumbing::get_query_impl
  38: rustc_interface::passes::QueryContext::enter
  39: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  40: rustc_span::with_source_map
  41: rustc_interface::interface::create_compiler_and_run
  42: scoped_tls::ScopedKey<T>::set
