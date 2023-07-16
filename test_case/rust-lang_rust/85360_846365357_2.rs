
stack backtrace:
   0: rust_begin_unwind
             at /rustc/5dc8789e300930751a78996da0fa906be5a344a2/library/std/src/panicking.rs:515:5
   1: std::panicking::begin_panic_fmt
             at /rustc/5dc8789e300930751a78996da0fa906be5a344a2/library/std/src/panicking.rs:457:5
   2: rustc_query_system::query::plumbing::incremental_verify_ich
   3: rustc_query_system::query::plumbing::load_from_disk_and_cache_in_memory
   4: rustc_data_structures::stack::ensure_sufficient_stack
   5: rustc_query_system::query::plumbing::get_query_impl
   6: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::evaluate_obligation
   7: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation
   8: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation_no_overflow
   9: rustc_trait_selection::traits::type_known_to_meet_bound_modulo_regions
  10: rustc_infer::infer::InferCtxtBuilder::enter
  11: rustc_ty_utils::common_traits::is_sized_raw
  12: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  13: rustc_query_system::query::plumbing::load_from_disk_and_cache_in_memory
  14: rustc_data_structures::stack::ensure_sufficient_stack
  15: rustc_query_system::query::plumbing::get_query_impl
  16: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::is_sized_raw
  17: rustc_middle::ty::util::<impl rustc_middle::ty::TyS>::is_sized
  18: rustc_mir::monomorphize::collector::find_vtable_types_for_unsizing::{{closure}}
  19: rustc_mir::monomorphize::collector::find_vtable_types_for_unsizing
  20: <rustc_mir::monomorphize::collector::MirNeighborCollector as rustc_middle::mir::visit::Visitor>::visit_rvalue
  21: rustc_mir::monomorphize::collector::collect_neighbours
  22: rustc_mir::monomorphize::collector::collect_items_rec
  23: rustc_mir::monomorphize::collector::collect_items_rec
  24: rustc_mir::monomorphize::collector::collect_items_rec
  25: rustc_mir::monomorphize::collector::collect_items_rec
  26: rustc_mir::monomorphize::collector::collect_items_rec
  27: rustc_session::utils::<impl rustc_session::session::Session>::time
  28: rustc_mir::monomorphize::collector::collect_crate_mono_items
  29: rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items
  30: rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::collect_and_partition_mono_items>::compute
  31: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  32: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  33: rustc_data_structures::stack::ensure_sufficient_stack
  34: rustc_query_system::query::plumbing::force_query_with_job
  35: rustc_query_system::query::plumbing::force_query_impl
  36: rustc_query_system::query::plumbing::force_query
  37: rustc_query_impl::query_callbacks::collect_and_partition_mono_items::force_from_dep_node
  38: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green
  39: rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_green_and_read
  40: rustc_data_structures::stack::ensure_sufficient_stack
  41: rustc_query_system::query::plumbing::get_query_impl
  42: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::exported_symbols
  43: rustc_metadata::rmeta::encoder::EncodeContext::encode_crate_root
  44: rustc_metadata::rmeta::encoder::encode_metadata_impl
  45: rustc_data_structures::sync::join
  46: rustc_metadata::rmeta::decoder::cstore_impl::<impl rustc_middle::middle::cstore::CrateStore for rustc_metadata::creader::CStore>::encode_metadata
  47: rustc_middle::ty::context::TyCtxt::encode_metadata
  48: rustc_interface::passes::QueryContext::enter
  49: rustc_interface::queries::Queries::ongoing_codegen
  50: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  51: rustc_span::with_source_map
  52: scoped_tls::ScopedKey<T>::set
