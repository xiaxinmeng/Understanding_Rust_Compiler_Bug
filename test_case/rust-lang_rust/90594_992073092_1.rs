
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_metadata/src/rmeta/def_path_hash_map.rs:18:85
stack backtrace:
   0: rust_begin_unwind
             at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/std/src/panicking.rs:517:5
   1: core::panicking::panic_fmt
             at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/core/src/panicking.rs:100:14
   2: core::panicking::panic
             at /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/library/core/src/panicking.rs:50:5
   3: rustc_metadata::rmeta::decoder::cstore_impl::<impl rustc_session::cstore::CrateStore for rustc_metadata::creader::CStore>::def_path_hash_to_def_id
   4: <rustc_query_impl::on_disk_cache::OnDiskCache as rustc_middle::ty::context::OnDiskCache>::def_path_hash_to_def_id
   5: rustc_middle::dep_graph::dep_node::<impl rustc_query_system::dep_graph::dep_node::DepNodeParams<rustc_middle::ty::context::TyCtxt> for rustc_span::def_id::DefId>::recover
   6: rustc_query_system::query::plumbing::force_query
   7: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
   8: rustc_query_system::query::plumbing::ensure_must_run
   9: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::check_mod_liveness
  10: rustc_middle::hir::map::Map::for_each_module
  11: rustc_session::utils::<impl rustc_session::session::Session>::time
  12: rustc_interface::passes::analysis
  13: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  14: rustc_data_structures::stack::ensure_sufficient_stack
  15: rustc_query_system::query::plumbing::try_execute_query
  16: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
  17: rustc_interface::passes::QueryContext::enter
  18: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  19: rustc_span::with_source_map
  20: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
