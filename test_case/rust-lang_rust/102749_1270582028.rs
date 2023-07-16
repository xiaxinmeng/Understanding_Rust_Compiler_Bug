
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_metadata/src/rmeta/def_path_hash_map.rs:23:85
stack backtrace:
   0: rust_begin_unwind
             at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/core/src/panicking.rs:142:14
   2: core::panicking::panic
             at /rustc/a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52/library/core/src/panicking.rs:48:5
   3: <rustc_metadata::creader::CStore as rustc_session::cstore::CrateStore>::def_path_hash_to_def_id
   4: <rustc_middle::ty::context::TyCtxt>::def_path_hash_to_def_id
   5: <rustc_query_system::dep_graph::dep_node::DepNode<rustc_middle::dep_graph::dep_node::DepKind> as rustc_middle::dep_graph::dep_node::DepNodeExt>::extract_def_id
   6: rustc_query_impl::query_callbacks::type_of::force_from_dep_node
   7: <rustc_middle::ty::context::TyCtxt as rustc_query_system::dep_graph::DepContext>::try_force_from_dep_node
   8: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
   9: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  10: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  11: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  12: rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::infer::canonical::Canonical<rustc_middle::ty::ParamEnvAnd<rustc_middle::ty::Predicate>>, core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>>
  13: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::evaluate_obligation, rustc_query_impl::plumbing::QueryCtxt>
  14: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::evaluate_obligation
  15: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation
  16: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation_no_overflow
  17: <rustc_trait_selection::traits::fulfill::FulfillProcessor>::process_trait_obligation
  18: <rustc_trait_selection::traits::fulfill::FulfillProcessor as rustc_data_structures::obligation_forest::ObligationProcessor>::process_obligation
  19: <rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection::traits::fulfill::FulfillProcessor, rustc_data_structures::obligation_forest::Outcome<rustc_trait_selection::traits::fulfill::PendingPredicateObligation, rustc_infer::traits::FulfillmentErrorCode>>
  20: <rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::select_where_possible
  21: <rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::select_all_or_error
  22: <rustc_trait_selection::traits::engine::ObligationCtxt>::select_all_or_error
  23: <rustc_infer::infer::InferCtxtBuilder>::enter::<(), rustc_typeck::check::wfcheck::enter_wf_checking_ctxt<rustc_typeck::check::wfcheck::check_impl::{closure#0}>::{closure#0}>
  24: rustc_typeck::check::wfcheck::check_well_formed
  25: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>
  26: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, ()>>
  27: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::check_well_formed, rustc_query_impl::plumbing::QueryCtxt>
  28: rustc_data_structures::sync::par_for_each_in::<&[rustc_hir::hir::ForeignItemId], <rustc_middle::hir::ModuleItems>::par_foreign_items<rustc_typeck::check::wfcheck::check_mod_type_wf::{closure#3}>::{closure#0}>
  29: rustc_typeck::check::wfcheck::check_mod_type_wf
  30: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, ()>
  31: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, ()>>
  32: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::check_mod_type_wf, rustc_query_impl::plumbing::QueryCtxt>
  33: <rustc_session::session::Session>::track_errors::<rustc_typeck::check_crate::{closure#5}, ()>
  34: rustc_typeck::check_crate
  35: rustc_interface::passes::analysis
  36: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>
  37: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorGuaranteed>>>
  38: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
  39: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
  40: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorGuaranteed>>
  41: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
  42: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0 (a55dd71d5 2022-09-19) running on aarch64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C lto=off -C embed-bitcode=no -C debuginfo=2 -C debug-assertions=on -C linker=clang -C incremental -C link-arg=-fuse-ld=/usr/bin/mold -C target-cpu=native

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `subgraph::UserComparison: core::marker::Sync`  |  = note: this failure-note originates in the attribute macro `entity` (in Nightly builds, run with -Z macro-backtrace for more info)

#1 [check_well_formed] checking that `subgraph::<impl at src/subgraph.rs:272:1: 277:3>` is well-formed
#2 [check_mod_type_wf] checking that types are well-formed in module `subgraph`
#3 [analysis] running analysis passes on this crate
end of query stack
