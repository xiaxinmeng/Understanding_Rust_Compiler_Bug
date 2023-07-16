
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /rustc/f1edd0429582dd29cccacaf50fd134b05593bd9c/compiler/rustc_hir/src/definitions.rs:452:14
stack backtrace:
   0: _rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::panic
   3: <rustc_query_impl::on_disk_cache::OnDiskCache as rustc_middle::ty::context::OnDiskCache>::def_path_hash_to_def_id
   4: rustc_middle::dep_graph::dep_node::<impl rustc_query_system::dep_graph::dep_node::DepNodeParams<rustc_middle::ty::context::TyCtxt> for rustc_span::def_id::LocalDefId>::recover
   5: rustc_query_impl::query_callbacks::hir_owner::force_from_dep_node
   6: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
   7: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
   8: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_green
   9: rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory
  10: rustc_query_system::query::plumbing::try_execute_query
  11: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::type_of
  12: <rustc_typeck::outlives::implicit_infer::InferVisitor as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_item
  13: rustc_typeck::outlives::inferred_outlives_crate
  14: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  15: rustc_data_structures::stack::ensure_sufficient_stack
  16: rustc_query_system::query::plumbing::try_execute_query
  17: rustc_query_system::query::plumbing::force_query_impl
  18: rustc_query_impl::query_callbacks::inferred_outlives_crate::force_from_dep_node
  19: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
  20: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
  21: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
  22: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
  23: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_green
  24: rustc_query_system::query::plumbing::ensure_must_run
  25: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::collect_mod_item_types
  26: rustc_middle::hir::map::Map::for_each_module
  27: rustc_session::session::Session::track_errors
  28: rustc_typeck::check_crate
  29: rustc_interface::passes::analysis
  30: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  31: rustc_data_structures::stack::ensure_sufficient_stack
  32: rustc_query_system::query::plumbing::try_execute_query
  33: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
  34: rustc_interface::passes::QueryContext::enter
  35: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  36: rustc_span::with_source_map
  37: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0 (f1edd0429 2021-11-29) running on x86_64-apple-darwin

note: compiler flags: -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [type_of] computing type of `asset::AssetKind::Links::links`
#1 [inferred_outlives_crate] computing the inferred outlives predicates for items in this crate
#2 [analysis] running analysis passes on this crate
end of query stack

