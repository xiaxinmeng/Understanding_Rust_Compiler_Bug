
   0: rust_begin_unwind
             at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/panicking.rs:517:5
   1: core::panicking::panic_fmt
             at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/core/src/panicking.rs:100:14
   2: core::panicking::panic
             at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/core/src/panicking.rs:50:5
   3: <rustc_query_impl::on_disk_cache::OnDiskCache as rustc_middle::ty::context::OnDiskCache>::def_path_hash_to_def_id
   4: rustc_middle::dep_graph::dep_node::<impl rustc_query_system::dep_graph::dep_node::DepNodeParams<rustc_middle::ty::context::TyCtxt> for rustc_span::def_id::LocalDefId>::recover
   5: rustc_query_impl::query_callbacks::hir_owner::force_from_dep_node
   6: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
   7: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
   8: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
   9: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
  10: rustc_query_system::query::plumbing::ensure_must_run
  11: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::coherent_trait
  12: rustc_session::session::Session::track_errors
  13: rustc_typeck::check_crate
  14: rustc_interface::passes::analysis
  15: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  16: rustc_data_structures::stack::ensure_sufficient_stack
  17: rustc_query_system::query::plumbing::try_execute_query
  18: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
  19: rustc_interface::passes::QueryContext::enter
  20: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  21: rustc_span::with_source_map
  22: scoped_tls::ScopedKey<T>::set
