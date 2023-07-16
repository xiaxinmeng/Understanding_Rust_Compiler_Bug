text
thread 'rustc' panicked at 'index out of bounds: the len is 82 but the index is 82', compiler/rustc_query_impl/src/on_disk_cache.rs:726:18
stack backtrace:
   0: rust_begin_unwind
             at /rustc/399ba6bb377ce02224b57c4d6e127e160fa76b34/library/std/src/panicking.rs:498:5
   1: core::panicking::panic_fmt
             at /rustc/399ba6bb377ce02224b57c4d6e127e160fa76b34/library/core/src/panicking.rs:107:14
   2: core::panicking::panic_bounds_check
             at /rustc/399ba6bb377ce02224b57c4d6e127e160fa76b34/library/core/src/panicking.rs:75:5
   3: <rustc_span::span_encoding::Span as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   4: <rustc_middle::ty::FieldDef as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   5: <rustc_middle::ty::VariantDef as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   6: <rustc_query_impl::on_disk_cache::CacheDecoder as rustc_serialize::serialize::Decoder>::read_seq::<alloc::vec::Vec<rustc_middle::ty::VariantDef>, <alloc::vec::Vec<rustc_middle::ty::VariantDef> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>
   7: <rustc_middle::ty::adt::AdtDef as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   8: <rustc_middle::ty::sty::TyKind as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   9: <&rustc_middle::ty::TyS as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  10: <&rustc_middle::ty::TyS as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  11: <core::result::Result<rustc_middle::ty::subst::GenericArg, alloc::string::String> as rustc_middle::ty::context::InternIteratorElement<rustc_middle::ty::subst::GenericArg, &rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>>::intern_with::<core::iter::adapters::map::Map<core::ops::range::Range<usize>, <&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>, <rustc_middle::ty::context::TyCtxt>::mk_substs<core::iter::adapters::map::Map<core::ops::range::Range<usize>, <&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>>::{closure#0}>
  12: <rustc_middle::ty::sty::TyKind as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  13: <&rustc_middle::ty::TyS as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  14: <rustc_query_impl::on_disk_cache::OnDiskCache>::try_load_query_result::<&rustc_middle::ty::TyS>
  15: rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, &rustc_middle::ty::TyS>
  16: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::type_of
  17: <rustc_typeck::outlives::implicit_infer::InferVisitor as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_item
  18: <rustc_middle::hir::map::Map>::visit_all_item_likes::<rustc_typeck::outlives::implicit_infer::InferVisitor>
  19: rustc_typeck::outlives::inferred_outlives_crate
  20: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), rustc_middle::ty::CratePredicatesMap>::{closure#0}, rustc_middle::ty::CratePredicatesMap>
  21: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, (), rustc_middle::ty::CratePredicatesMap>
  22: rustc_data_structures::stack::ensure_sufficient_stack::<(rustc_middle::ty::CratePredicatesMap, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), rustc_middle::ty::CratePredicatesMap>::{closure#3}>
  23: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::ArenaCache<(), rustc_middle::ty::CratePredicatesMap>>
  24: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::inferred_outlives_crate, rustc_query_impl::plumbing::QueryCtxt>
  25: rustc_typeck::outlives::inferred_outlives_of
  26: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::DefId, &[(rustc_middle::ty::Predicate, rustc_span::span_encoding::Span)]>
  27: rustc_data_structures::stack::ensure_sufficient_stack::<(&[(rustc_middle::ty::Predicate, rustc_span::span_encoding::Span)], rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, &[(rustc_middle::ty::Predicate, rustc_span::span_encoding::Span)]>::{closure#3}>
  28: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, &[(rustc_middle::ty::Predicate, rustc_span::span_encoding::Span)]>>
  29: rustc_query_system::query::plumbing::force_query::<rustc_query_impl::queries::inferred_outlives_of, rustc_query_impl::plumbing::QueryCtxt>
  30: rustc_query_impl::query_callbacks::inferred_outlives_of::force_from_dep_node
  31: <rustc_middle::ty::context::TyCtxt as rustc_query_system::dep_graph::DepContext>::try_force_from_dep_node
  32: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  33: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  34: rustc_query_system::query::plumbing::ensure_must_run::<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, ()>
  35: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::predicates_of
  36: <rustc_typeck::collect::CollectItemTypesVisitor as rustc_hir::intravisit::Visitor>::visit_item
  37: <rustc_middle::hir::map::Map>::visit_item_likes_in_module::<rustc_hir::intravisit::DeepVisitor<rustc_typeck::collect::CollectItemTypesVisitor>>
  38: rustc_typeck::collect::collect_mod_item_types
  39: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>
  40: rustc_data_structures::stack::ensure_sufficient_stack::<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, ()>::{closure#3}>
  41: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, ()>>
  42: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::collect_mod_item_types, rustc_query_impl::plumbing::QueryCtxt>
  43: <rustc_middle::hir::map::Map>::for_each_module::<rustc_typeck::check_crate::{closure#0}::{closure#0}::{closure#0}>
  44: <rustc_session::session::Session>::track_errors::<rustc_typeck::check_crate::{closure#0}, ()>
  45: rustc_typeck::check_crate
  46: rustc_interface::passes::analysis
  47: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>
  48: rustc_data_structures::stack::ensure_sufficient_stack::<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>
  49: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorReported>>>
  50: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
  51: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorReported>>
  52: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorReported>>
  53: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
  54: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.59.0-nightly (399ba6bb3 2022-01-03) running on x86_64-unknown-linux-gnu
note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C linker=clang -C incremental -C link-arg=-fuse-ld=/usr/bin/mold --crate-type lib
note: some of the compiler flags provided by cargo are hidden
query stack during panic:
#0 [type_of] computing type of `controller::PublicApiController::indicator`
#1 [inferred_outlives_crate] computing the inferred outlives predicates for items in this crate
#2 [inferred_outlives_of] computing inferred outlives predicates of `SubscribeManager`
#3 [collect_mod_item_types] collecting item types in top-level module
#4 [analysis] running analysis passes on this crate
end of query stack
