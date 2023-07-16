
 thread 'rustc' panicked at 'found unstable fingerprints for extern_mod_stmt_cnum(rcgen[846b]::rcgen)', /rustc/acca818928654807ed3bc1ce0e97df118f8716c8/compiler/rustc_query_system/src/query/plumbing.rs:593:5
stack backtrace:
   0: _rust_begin_unwind
   1: std::panicking::begin_panic_fmt
   2: rustc_query_system::query::plumbing::incremental_verify_ich
   3: rustc_query_system::query::plumbing::load_from_disk_and_cache_in_memory
   4: rustc_data_structures::stack::ensure_sufficient_stack
   5: rustc_query_system::query::plumbing::get_query_impl
   6: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::extern_mod_stmt_cnum
   7: <rustc_passes::stability::Checker as rustc_hir::intravisit::Visitor>::visit_item
   8: rustc_middle::hir::map::Map::visit_item_likes_in_module
   9: rustc_passes::stability::check_mod_unstable_api_usage
  10: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  11: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  12: rustc_data_structures::stack::ensure_sufficient_stack
  13: rustc_query_system::query::plumbing::force_query_with_job
  14: rustc_query_system::query::plumbing::get_query_impl
  15: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::check_mod_unstable_api_usage
  16: std::panic::catch_unwind
  17: rustc_session::utils::<impl rustc_session::session::Session>::time
  18: rustc_interface::passes::analysis
  19: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  20: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  21: rustc_data_structures::stack::ensure_sufficient_stack
  22: rustc_query_system::query::plumbing::force_query_with_job
  23: rustc_query_system::query::plumbing::get_query_impl
  24: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
  25: rustc_interface::passes::QueryContext::enter
  26: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  27: rustc_span::with_source_map
  28: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
