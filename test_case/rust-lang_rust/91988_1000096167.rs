
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
   8: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
   9: rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory
  10: rustc_query_system::query::plumbing::try_execute_query
  11: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::unsafety_check_result
  12: <rustc_mir_transform::check_unsafety::UnsafetyChecker as rustc_middle::mir::visit::Visitor>::visit_rvalue
  13: rustc_mir_transform::check_unsafety::unsafety_check_result
  14: core::ops::function::FnOnce::call_once
  15: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  16: rustc_data_structures::stack::ensure_sufficient_stack
  17: rustc_query_system::query::plumbing::try_execute_query
  18: rustc_query_system::query::plumbing::force_query_impl
  19: rustc_query_impl::query_callbacks::unsafety_check_result::force_from_dep_node
  20: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
  21: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
  22: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
  23: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
  24: rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory
  25: rustc_query_system::query::plumbing::try_execute_query
  26: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::type_of
  27: rustc_typeck::check::check::check_item_type
  28: rustc_middle::hir::map::Map::visit_item_likes_in_module
  29: rustc_typeck::check::check::check_mod_item_types
  30: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  31: rustc_data_structures::stack::ensure_sufficient_stack
  32: rustc_query_system::query::plumbing::try_execute_query
  33: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::check_mod_item_types
  34: rustc_middle::hir::map::Map::for_each_module
  35: rustc_typeck::check_crate
  36: rustc_interface::passes::analysis
  37: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  38: rustc_data_structures::stack::ensure_sufficient_stack
  39: rustc_query_system::query::plumbing::try_execute_query
  40: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
  41: rustc_interface::passes::QueryContext::enter
  42: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  43: rustc_span::with_source_map
  44: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.57.0 (f1edd0429 2021-11-29) running on x86_64-unknown-linux-gnu
note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib
note: some of the compiler flags provided by cargo are hidden
query stack during panic:
