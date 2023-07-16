
thread 'rustc' panicked at 'Failed to extract DefId: local_def_id_to_hir_id e3b2d0908ae3c069-bf4fac173a9fa23', compiler/rustc_middle/src/dep_graph/dep_node.rs:273:17
stack backtrace:
   0: rust_begin_unwind
             at /rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f/library/core/src/panicking.rs:142:14
   2: <rustc_query_system::dep_graph::dep_node::DepNode<rustc_middle::dep_graph::dep_node::DepKind> as rustc_middle::dep_graph::dep_node::DepNodeExt>::extract_def_id::{closure#0}
   3: <rustc_query_system::dep_graph::dep_node::DepNode<rustc_middle::dep_graph::dep_node::DepKind> as rustc_middle::dep_graph::dep_node::DepNodeExt>::extract_def_id
   4: rustc_query_impl::query_callbacks::local_def_id_to_hir_id::force_from_dep_node
   5: <rustc_middle::ty::context::TyCtxt as rustc_query_system::dep_graph::DepContext>::try_force_from_dep_node
   6: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
   7: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
   8: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
   9: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  10: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  11: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  12: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  13: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  14: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
  15: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::evaluate_obligation, rustc_query_impl::plumbing::QueryCtxt>
  16: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::evaluate_obligation
  17: <rustc_trait_selection::traits::fulfill::FulfillProcessor as rustc_data_structures::obligation_forest::ObligationProcessor>::process_obligation
  18: <rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection::traits::fulfill::FulfillProcessor, rustc_data_structures::obligation_forest::Outcome<rustc_trait_selection::traits::fulfill::PendingPredicateObligation, rustc_infer::traits::FulfillmentErrorCode>>
  19: <rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::select_where_possible
  20: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_argument_types
  21: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_call
  22: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
  23: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  24: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_stmt
  25: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_block_with_expected
  26: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
  27: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  28: rustc_typeck::check::check::check_fn
  29: <rustc_infer::infer::InferCtxtBuilder>::enter::<&rustc_middle::ty::context::TypeckResults, <rustc_typeck::check::inherited::InheritedBuilder>::enter<rustc_typeck::check::typeck_with_fallback<rustc_typeck::check::typeck::{closure#0}>::{closure#1}, &rustc_middle::ty::context::TypeckResults>::{closure#0}>
  30: rustc_typeck::check::typeck
  31: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>
  32: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>>
  33: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::typeck
  34: <rustc_middle::hir::map::Map>::par_body_owners::<rustc_typeck::check::typeck_item_bodies::{closure#0}>
  35: rustc_typeck::check::typeck_item_bodies
  36: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, (), ()>
  37: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), ()>>
  38: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::typeck_item_bodies, rustc_query_impl::plumbing::QueryCtxt>
  39: <rustc_session::session::Session>::time::<(), rustc_typeck::check_crate::{closure#7}>
  40: rustc_typeck::check_crate
  41: rustc_interface::passes::analysis
  42: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorGuaranteed>>
  43: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorGuaranteed>>>
  44: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
  45: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
  46: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorGuaranteed>>
  47: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
  48: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0 (4b91a6ea7 2022-08-08) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C embed-bitcode=no -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `(): AsQuery`
#1 [typeck] type-checking `foo`
#2 [typeck_item_bodies] type-checking all item bodies
#3 [analysis] running analysis passes on this crate
end of query stack
