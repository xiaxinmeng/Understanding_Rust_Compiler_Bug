
thread 'rustc' panicked at 'failed to lookup `SourceFile` in new context', compiler/rustc_query_impl/src/on_disk_cache.rs:514:22
stack backtrace:
   0: rust_begin_unwind
             at /rustc/cfa4ac66c194046f631ce076c75516ecfdeb77ee/library/std/src/panicking.rs:498:5
   1: core::panicking::panic_fmt
             at /rustc/cfa4ac66c194046f631ce076c75516ecfdeb77ee/library/core/src/panicking.rs:107:14
   2: core::panicking::panic_display
             at /rustc/cfa4ac66c194046f631ce076c75516ecfdeb77ee/library/core/src/panicking.rs:63:5
   3: core::panicking::panic_str
             at /rustc/cfa4ac66c194046f631ce076c75516ecfdeb77ee/library/core/src/panicking.rs:55:5
   4: core::option::expect_failed
             at /rustc/cfa4ac66c194046f631ce076c75516ecfdeb77ee/library/core/src/option.rs:1817:5
   5: <rustc_span::span_encoding::Span as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   6: <rustc_middle::ty::VariantDef as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   7: <rustc_query_impl::on_disk_cache::CacheDecoder as rustc_serialize::serialize::Decoder>::read_seq::<alloc::vec::Vec<rustc_middle::ty::VariantDef>, <alloc::vec::Vec<rustc_middle::ty::VariantDef> as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode::{closure#0}>
   8: <rustc_middle::ty::adt::AdtDef as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
   9: <rustc_middle::ty::sty::TyKind as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  10: <&rustc_middle::ty::TyS as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  11: <&rustc_middle::ty::TyS as rustc_serialize::serialize::Decodable<rustc_query_impl::on_disk_cache::CacheDecoder>>::decode
  12: <rustc_query_impl::on_disk_cache::OnDiskCache>::try_load_query_result::<&rustc_middle::ty::TyS>
  13: rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::DefId, &rustc_middle::ty::TyS>
  14: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::type_of
  15: <rustc_typeck::outlives::implicit_infer::InferVisitor as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_item
  16: <rustc_middle::hir::map::Map>::visit_all_item_likes::<rustc_typeck::outlives::implicit_infer::InferVisitor>
  17: rustc_typeck::outlives::inferred_outlives_crate
  18: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), rustc_middle::ty::CratePredicatesMap>::{closure#0}, rustc_middle::ty::CratePredicatesMap>
  19: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, (), rustc_middle::ty::CratePredicatesMap>
  20: rustc_data_structures::stack::ensure_sufficient_stack::<(rustc_middle::ty::CratePredicatesMap, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), rustc_middle::ty::CratePredicatesMap>::{closure#3}>
  21: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::ArenaCache<(), rustc_middle::ty::CratePredicatesMap>>
  22: rustc_query_system::query::plumbing::force_query::<rustc_query_impl::queries::inferred_outlives_crate, rustc_query_impl::plumbing::QueryCtxt>
  23: rustc_query_impl::query_callbacks::inferred_outlives_crate::force_from_dep_node
  24: <rustc_middle::ty::context::TyCtxt as rustc_query_system::dep_graph::DepContext>::try_force_from_dep_node
  25: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  26: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  27: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  28: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  29: rustc_query_system::query::plumbing::ensure_must_run::<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::mir::query::UnsafetyCheckResult>
  30: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::collect_mod_item_types, rustc_query_impl::plumbing::QueryCtxt>
  31: <rustc_middle::hir::map::Map>::for_each_module::<rustc_typeck::check_crate::{closure#0}::{closure#0}::{closure#0}>
  32: <rustc_session::session::Session>::track_errors::<rustc_typeck::check_crate::{closure#0}, ()>
  33: rustc_typeck::check_crate
  34: rustc_interface::passes::analysis
  35: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>
  36: rustc_data_structures::stack::ensure_sufficient_stack::<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>
  37: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorReported>>>
  38: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
  39: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorReported>>
  40: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
  41: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>
  42: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0-nightly (cfa4ac66c 2022-01-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type cdylib --crate-type rlib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [type_of] computing type of `python::minimizers::cost_fn::PyHilberSchmidtCostFn::cost_fn`
#1 [inferred_outlives_crate] computing the inferred outlives predicates for items in this crate
#2 [analysis] running analysis passes on this crate
end of query stack
