
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /home/aaron/repos/rust/compiler/rustc_hir/src/definitions.rs:458:14
stack backtrace:
   0: rust_begin_unwind
             at /home/aaron/repos/rust/library/std/src/panicking.rs:577:5
   1: core::panicking::panic_fmt
             at /home/aaron/repos/rust/library/core/src/panicking.rs:110:14
   2: core::panicking::panic
             at /home/aaron/repos/rust/library/core/src/panicking.rs:48:5
   3: <core::option::Option<rustc_span::def_id::LocalDefId>>::unwrap
             at /home/aaron/repos/rust/library/core/src/option.rs:729:21
   4: <rustc_hir::definitions::Definitions>::local_def_path_hash_to_def_id
             at /home/aaron/repos/rust/compiler/rustc_hir/src/definitions.rs:454:9
   5: <rustc_middle::ty::context::TyCtxt>::def_path_hash_to_def_id
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1319:13
   6: <rustc_query_system::dep_graph::dep_node::DepNode<rustc_middle::dep_graph::dep_node::DepKind> as rustc_middle::dep_graph::dep_node::DepNodeExt>::extract_def_id
             at /home/aaron/repos/rust/compiler/rustc_middle/src/dep_graph/dep_node.rs:269:18
   7: <rustc_span::def_id::DefId as rustc_query_system::dep_graph::dep_node::DepNodeParams<rustc_middle::ty::context::TyCtxt>>::recover
             at /home/aaron/repos/rust/compiler/rustc_middle/src/dep_graph/dep_node.rs:333:9
   8: rustc_query_impl::query_callbacks::type_of::recover
             at /home/aaron/repos/rust/compiler/rustc_query_impl/src/plumbing.rs:424:21
   9: rustc_query_impl::query_callbacks::type_of::force_from_dep_node
             at /home/aaron/repos/rust/compiler/rustc_query_impl/src/plumbing.rs:428:40
  10: <rustc_middle::ty::context::TyCtxt as rustc_query_system::dep_graph::DepContext>::try_force_from_dep_node
             at /home/aaron/repos/rust/compiler/rustc_middle/src/dep_graph/mod.rs:126:13
  11: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_parent_green::<rustc_query_impl::plumbing::QueryCtxt>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:648:13
  12: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:722:13
  13: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_parent_green::<rustc_query_impl::plumbing::QueryCtxt>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:633:17
  14: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl::plumbing::QueryCtxt>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:722:13
  15: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::try_mark_green::<rustc_query_impl::plumbing::QueryCtxt>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:581:17
  16: rustc_query_system::query::plumbing::try_load_from_disk_and_cache_in_memory::<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::infer::canonical::Canonical<rustc_middle::ty::ParamEnvAnd<rustc_middle::ty::Predicate>>, core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/plumbing.rs:509:49
  17: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<core::option::Option<(core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::infer::canonical::Canonical<rustc_middle::ty::ParamEnvAnd<rustc_middle::ty::Predicate>>, core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>>::{closure#2}>::{closure#0}::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_query_impl/src/plumbing.rs:103:17
  18: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::infer::canonical::Canonical<rustc_middle::ty::ParamEnvAnd<rustc_middle::ty::Predicate>>, core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>>::{closure#2}>::{closure#0}::{closure#0}, core::option::Option<(core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1764:50
  19: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::infer::canonical::Canonical<rustc_middle::ty::ParamEnvAnd<rustc_middle::ty::Predicate>>, core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>>::{closure#2}>::{closure#0}::{closure#0}, core::option::Option<(core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}, core::option::Option<(core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>, rustc_query_system::dep_graph::graph::DepNodeIndex)>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1748:9
  20: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::infer::canonical::Canonical<rustc_middle::ty::ParamEnvAnd<rustc_middle::ty::Predicate>>, core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>>::{closure#2}>::{closure#0}::{closure#0}, core::option::Option<(core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>, rustc_query_system::dep_graph::graph::DepNodeIndex)>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1764:9
  21: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<core::option::Option<(core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::infer::canonical::Canonical<rustc_middle::ty::ParamEnvAnd<rustc_middle::ty::Predicate>>, core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>>::{closure#2}>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_query_impl/src/plumbing.rs:102:13
  22: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::infer::canonical::Canonical<rustc_middle::ty::ParamEnvAnd<rustc_middle::ty::Predicate>>, core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>>::{closure#2}>::{closure#0}, core::option::Option<(core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1808:13
  23: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::infer::canonical::Canonical<rustc_middle::ty::ParamEnvAnd<rustc_middle::ty::Predicate>>, core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>>::{closure#2}>::{closure#0}, core::option::Option<(core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}, core::option::Option<(core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1792:40
  24: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::infer::canonical::Canonical<rustc_middle::ty::ParamEnvAnd<rustc_middle::ty::Predicate>>, core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>>::{closure#2}>::{closure#0}, core::option::Option<(core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}, core::option::Option<(core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}, core::option::Option<(core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>, rustc_query_system::dep_graph::graph::DepNodeIndex)>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1781:22
  25: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::infer::canonical::Canonical<rustc_middle::ty::ParamEnvAnd<rustc_middle::ty::Predicate>>, core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>>::{closure#2}>::{closure#0}, core::option::Option<(core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>, rustc_query_system::dep_graph::graph::DepNodeIndex)>>::{closure#0}, core::option::Option<(core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>, rustc_query_system::dep_graph::graph::DepNodeIndex)>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1792:9
  26: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<core::option::Option<(core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::infer::canonical::Canonical<rustc_middle::ty::ParamEnvAnd<rustc_middle::ty::Predicate>>, core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>>::{closure#2}>::{closure#0}, core::option::Option<(core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>, rustc_query_system::dep_graph::graph::DepNodeIndex)>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1805:9
  27: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<core::option::Option<(core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>, rustc_query_system::dep_graph::graph::DepNodeIndex)>, rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::infer::canonical::Canonical<rustc_middle::ty::ParamEnvAnd<rustc_middle::ty::Predicate>>, core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>>::{closure#2}>
             at /home/aaron/repos/rust/compiler/rustc_query_impl/src/plumbing.rs:91:9
  28: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, rustc_middle::infer::canonical::Canonical<rustc_middle::ty::ParamEnvAnd<rustc_middle::ty::Predicate>>, core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/plumbing.rs:455:28
  29: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_middle::infer::canonical::Canonical<rustc_middle::ty::ParamEnvAnd<rustc_middle::ty::Predicate>>, core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>>>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/plumbing.rs:400:44
  30: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::evaluate_obligation, rustc_query_impl::plumbing::QueryCtxt>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/plumbing.rs:765:36
  31: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::evaluate_obligation
             at /home/aaron/repos/rust/compiler/rustc_query_impl/src/plumbing.rs:537:17
  32: <rustc_middle::ty::query::TyCtxtAt>::evaluate_obligation
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/query.rs:254:17
  33: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation
             at /home/aaron/repos/rust/compiler/rustc_trait_selection/src/traits/query/evaluate_obligation.rs:87:9
  34: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation_no_overflow
             at /home/aaron/repos/rust/compiler/rustc_trait_selection/src/traits/query/evaluate_obligation.rs:97:15
  35: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::evaluate_obligation::InferCtxtExt>::predicate_must_hold_considering_regions
             at /home/aaron/repos/rust/compiler/rustc_trait_selection/src/traits/query/evaluate_obligation.rs:51:9
  36: <rustc_trait_selection::traits::fulfill::FulfillProcessor>::process_trait_obligation
             at /home/aaron/repos/rust/compiler/rustc_trait_selection/src/traits/fulfill.rs:660:16
  37: <rustc_trait_selection::traits::fulfill::FulfillProcessor>::progress_changed_obligations
             at /home/aaron/repos/rust/compiler/rustc_trait_selection/src/traits/fulfill.rs:405:21
  38: <rustc_trait_selection::traits::fulfill::FulfillProcessor as rustc_data_structures::obligation_forest::ObligationProcessor>::process_obligation
             at /home/aaron/repos/rust/compiler/rustc_trait_selection/src/traits/fulfill.rs:317:9
  39: <rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection::traits::fulfill::FulfillProcessor, rustc_data_structures::obligation_forest::Outcome<rustc_trait_selection::traits::fulfill::PendingPredicateObligation, rustc_infer::traits::FulfillmentErrorCode>>
             at /home/aaron/repos/rust/compiler/rustc_data_structures/src/obligation_forest/mod.rs:449:19
  40: <rustc_trait_selection::traits::fulfill::FulfillmentContext>::select
             at /home/aaron/repos/rust/compiler/rustc_trait_selection/src/traits/fulfill.rs:142:17
  41: <rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::select_where_possible
             at /home/aaron/repos/rust/compiler/rustc_trait_selection/src/traits/fulfill.rs:241:9
  42: <rustc_typeck::check::fn_ctxt::FnCtxt>::select_obligations_where_possible::<<rustc_typeck::check::fn_ctxt::FnCtxt>::check_argument_types::{closure#4}>
             at /home/aaron/repos/rust/compiler/rustc_typeck/src/check/fn_ctxt/_impl.rs:636:26
  43: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_argument_types
             at /home/aaron/repos/rust/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs:380:17
  44: <rustc_typeck::check::fn_ctxt::FnCtxt>::confirm_builtin_call
             at /home/aaron/repos/rust/compiler/rustc_typeck/src/check/callee.rs:495:9
  45: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_call
  46: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
             at /home/aaron/repos/rust/compiler/rustc_typeck/src/check/expr.rs:308:45
  47: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
             at /home/aaron/repos/rust/compiler/rustc_typeck/src/check/expr.rs:214:18
  48: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
             at /home/aaron/repos/rust/compiler/rustc_typeck/src/check/expr.rs:167:9
  49: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_stmt
             at /home/aaron/repos/rust/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs:649:17
  50: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_block_with_expected::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs:709:17
  51: <rustc_typeck::check::fn_ctxt::FnCtxt>::with_breakable_ctxt::<<rustc_typeck::check::fn_ctxt::FnCtxt>::check_block_with_expected::{closure#0}, ()>
             at /home/aaron/repos/rust/compiler/rustc_typeck/src/check/fn_ctxt/_impl.rs:1519:22
  52: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_block_with_expected
             at /home/aaron/repos/rust/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs:707:26
  53: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
             at /home/aaron/repos/rust/compiler/rustc_typeck/src/check/expr.rs:307:41
  54: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
             at /home/aaron/repos/rust/compiler/rustc_typeck/src/check/expr.rs:214:18
  55: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
             at /home/aaron/repos/rust/compiler/rustc_typeck/src/check/expr.rs:167:9
  56: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_hint
             at /home/aaron/repos/rust/compiler/rustc_typeck/src/check/expr.rs:119:9
  57: <rustc_typeck::check::fn_ctxt::FnCtxt>::check_return_expr
             at /home/aaron/repos/rust/compiler/rustc_typeck/src/check/expr.rs:809:30
  58: rustc_typeck::check::check::check_fn
             at /home/aaron/repos/rust/compiler/rustc_typeck/src/check/check.rs:216:9
  59: rustc_typeck::check::typeck_with_fallback::<rustc_typeck::check::typeck::{closure#0}>::{closure#1}
             at /home/aaron/repos/rust/compiler/rustc_typeck/src/check/mod.rs:402:23
  60: <rustc_typeck::check::inherited::InheritedBuilder>::enter::<rustc_typeck::check::typeck_with_fallback<rustc_typeck::check::typeck::{closure#0}>::{closure#1}, &rustc_middle::ty::context::TypeckResults>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_typeck/src/check/inherited.rs:96:34
  61: <rustc_infer::infer::InferCtxtBuilder>::enter::<&rustc_middle::ty::context::TypeckResults, <rustc_typeck::check::inherited::InheritedBuilder>::enter<rustc_typeck::check::typeck_with_fallback<rustc_typeck::check::typeck::{closure#0}>::{closure#1}, &rustc_middle::ty::context::TypeckResults>::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_infer/src/infer/mod.rs:613:9
  62: <rustc_typeck::check::inherited::InheritedBuilder>::enter::<rustc_typeck::check::typeck_with_fallback<rustc_typeck::check::typeck::{closure#0}>::{closure#1}, &rustc_middle::ty::context::TypeckResults>
             at /home/aaron/repos/rust/compiler/rustc_typeck/src/check/inherited.rs:96:9
  63: rustc_typeck::check::typeck_with_fallback::<rustc_typeck::check::typeck::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_typeck/src/check/mod.rs:365:26
  64: rustc_typeck::check::typeck
             at /home/aaron/repos/rust/compiler/rustc_typeck/src/check/mod.rs:330:9
  65: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:326:53
  66: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#0}, &rustc_middle::ty::context::TypeckResults>::{closure#0}::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/dep_graph/mod.rs:55:46
  67: rustc_middle::ty::context::tls::enter_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#0}, &rustc_middle::ty::context::TypeckResults>::{closure#0}::{closure#0}, &rustc_middle::ty::context::TypeckResults>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1764:50
  68: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#0}, &rustc_middle::ty::context::TypeckResults>::{closure#0}::{closure#0}, &rustc_middle::ty::context::TypeckResults>::{closure#0}, &rustc_middle::ty::context::TypeckResults>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1748:9
  69: rustc_middle::ty::context::tls::enter_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#0}, &rustc_middle::ty::context::TypeckResults>::{closure#0}::{closure#0}, &rustc_middle::ty::context::TypeckResults>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1764:9
  70: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#0}, &rustc_middle::ty::context::TypeckResults>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/dep_graph/mod.rs:55:13
  71: rustc_middle::ty::context::tls::with_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#0}, &rustc_middle::ty::context::TypeckResults>::{closure#0}, &rustc_middle::ty::context::TypeckResults>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1792:40
  72: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#0}, &rustc_middle::ty::context::TypeckResults>::{closure#0}, &rustc_middle::ty::context::TypeckResults>::{closure#0}, &rustc_middle::ty::context::TypeckResults>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1781:22
  73: rustc_middle::ty::context::tls::with_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#0}, &rustc_middle::ty::context::TypeckResults>::{closure#0}, &rustc_middle::ty::context::TypeckResults>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1792:9
  74: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#0}, &rustc_middle::ty::context::TypeckResults>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/dep_graph/mod.rs:52:9
  75: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:326:22
  76: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:274:13
  77: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#3}
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/plumbing.rs:475:9
  78: stacker::maybe_grow::<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#3}>
             at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.14/src/lib.rs:55:9
  79: rustc_data_structures::stack::ensure_sufficient_stack::<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#3}>
             at /home/aaron/repos/rust/compiler/rustc_data_structures/src/stack.rs:16:5
  80: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#3}>::{closure#0}::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_query_impl/src/plumbing.rs:103:17
  81: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#3}>::{closure#0}::{closure#0}, (&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1764:50
  82: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#3}>::{closure#0}::{closure#0}, (&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, (&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1748:9
  83: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#3}>::{closure#0}::{closure#0}, (&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1764:9
  84: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#3}>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_query_impl/src/plumbing.rs:102:13
  85: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#3}>::{closure#0}, (&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1808:13
  86: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#3}>::{closure#0}, (&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, (&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1792:40
  87: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#3}>::{closure#0}, (&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, (&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, (&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1781:22
  88: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#3}>::{closure#0}, (&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, (&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1792:9
  89: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#3}>::{closure#0}, (&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1805:9
  90: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<(&rustc_middle::ty::context::TypeckResults, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>::{closure#3}>
             at /home/aaron/repos/rust/compiler/rustc_query_impl/src/plumbing.rs:91:9
  91: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/plumbing.rs:465:36
  92: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::LocalDefId, &rustc_middle::ty::context::TypeckResults>>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/plumbing.rs:400:44
  93: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::typeck, rustc_query_impl::plumbing::QueryCtxt>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/plumbing.rs:765:36
  94: <rustc_middle::ty::query::TyCtxtEnsure>::typeck
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/query.rs:225:17
  95: rustc_typeck::check::typeck_item_bodies::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_typeck/src/check/mod.rs:944:51
  96: <rustc_middle::hir::map::Map>::par_body_owners::<rustc_typeck::check::typeck_item_bodies::{closure#0}>::{closure#0}::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/hir/map/mod.rs:543:21
  97: <core::slice::iter::Iter<(rustc_hir::hir_id::ItemLocalId, &rustc_hir::hir::Body)> as core::iter::traits::iterator::Iterator>::for_each::<<rustc_middle::hir::map::Map>::par_body_owners<rustc_typeck::check::typeck_item_bodies::{closure#0}>::{closure#0}::{closure#0}>
             at /home/aaron/repos/rust/library/core/src/slice/iter/macros.rs:211:21
  98: <rustc_middle::hir::map::Map>::par_body_owners::<rustc_typeck::check::typeck_item_bodies::{closure#0}>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/hir/map/mod.rs:540:17
  99: core::iter::traits::iterator::Iterator::for_each::call::<(usize, &core::option::Option<rustc_hir::hir::OwnerInfo>), <rustc_middle::hir::map::Map>::par_body_owners<rustc_typeck::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}
             at /home/aaron/repos/rust/library/core/src/iter/traits/iterator.rs:733:29
 100: <core::iter::adapters::enumerate::Enumerate<_> as core::iter::traits::iterator::Iterator>::fold::enumerate::<&core::option::Option<rustc_hir::hir::OwnerInfo>, (), core::iter::traits::iterator::Iterator::for_each::call<(usize, &core::option::Option<rustc_hir::hir::OwnerInfo>), <rustc_middle::hir::map::Map>::par_body_owners<rustc_typeck::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}>::{closure#0}
             at /home/aaron/repos/rust/library/core/src/iter/adapters/enumerate.rs:106:27
 101: <core::slice::iter::Iter<core::option::Option<rustc_hir::hir::OwnerInfo>> as core::iter::traits::iterator::Iterator>::fold::<(), <core::iter::adapters::enumerate::Enumerate<_> as core::iter::traits::iterator::Iterator>::fold::enumerate<&core::option::Option<rustc_hir::hir::OwnerInfo>, (), core::iter::traits::iterator::Iterator::for_each::call<(usize, &core::option::Option<rustc_hir::hir::OwnerInfo>), <rustc_middle::hir::map::Map>::par_body_owners<rustc_typeck::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}>::{closure#0}>
             at /home/aaron/repos/rust/library/core/src/iter/traits/iterator.rs:2167:21
 102: <core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<core::option::Option<rustc_hir::hir::OwnerInfo>>> as core::iter::traits::iterator::Iterator>::fold::<(), core::iter::traits::iterator::Iterator::for_each::call<(usize, &core::option::Option<rustc_hir::hir::OwnerInfo>), <rustc_middle::hir::map::Map>::par_body_owners<rustc_typeck::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}>
             at /home/aaron/repos/rust/library/core/src/iter/adapters/enumerate.rs:112:9
 103: <core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<core::option::Option<rustc_hir::hir::OwnerInfo>>> as core::iter::traits::iterator::Iterator>::for_each::<<rustc_middle::hir::map::Map>::par_body_owners<rustc_typeck::check::typeck_item_bodies::{closure#0}>::{closure#0}>
             at /home/aaron/repos/rust/library/core/src/iter/traits/iterator.rs:736:9
 104: <rustc_middle::hir::map::Map>::par_body_owners::<rustc_typeck::check::typeck_item_bodies::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/hir/map/mod.rs:537:9
 105: rustc_typeck::check::typeck_item_bodies
             at /home/aaron/repos/rust/compiler/rustc_typeck/src/check/mod.rs:944:5
 106: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl::<rustc_middle::ty::context::TyCtxt, (), ()>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:326:53
 107: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), ()>::{closure#0}, ()>::{closure#0}::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/dep_graph/mod.rs:55:46
 108: rustc_middle::ty::context::tls::enter_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), ()>::{closure#0}, ()>::{closure#0}::{closure#0}, ()>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1764:50
 109: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), ()>::{closure#0}, ()>::{closure#0}::{closure#0}, ()>::{closure#0}, ()>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1748:9
 110: rustc_middle::ty::context::tls::enter_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), ()>::{closure#0}, ()>::{closure#0}::{closure#0}, ()>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1764:9
 111: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), ()>::{closure#0}, ()>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/dep_graph/mod.rs:55:13
 112: rustc_middle::ty::context::tls::with_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), ()>::{closure#0}, ()>::{closure#0}, ()>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1792:40
 113: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), ()>::{closure#0}, ()>::{closure#0}, ()>::{closure#0}, ()>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1781:22
 114: rustc_middle::ty::context::tls::with_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), ()>::{closure#0}, ()>::{closure#0}, ()>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1792:9
 115: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), ()>::{closure#0}, ()>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/dep_graph/mod.rs:52:9
 116: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl::<rustc_middle::ty::context::TyCtxt, (), ()>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:326:22
 117: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, (), ()>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:274:13
 118: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#3}
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/plumbing.rs:475:9
 119: stacker::maybe_grow::<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#3}>
             at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.14/src/lib.rs:55:9
 120: rustc_data_structures::stack::ensure_sufficient_stack::<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#3}>
             at /home/aaron/repos/rust/compiler/rustc_data_structures/src/stack.rs:16:5
 121: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#3}>::{closure#0}::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_query_impl/src/plumbing.rs:103:17
 122: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#3}>::{closure#0}::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1764:50
 123: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#3}>::{closure#0}::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1748:9
 124: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#3}>::{closure#0}::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1764:9
 125: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#3}>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_query_impl/src/plumbing.rs:102:13
 126: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#3}>::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1808:13
 127: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#3}>::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1792:40
 128: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#3}>::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1781:22
 129: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#3}>::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1792:9
 130: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#3}>::{closure#0}, ((), rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1805:9
 131: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<((), rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), ()>::{closure#3}>
             at /home/aaron/repos/rust/compiler/rustc_query_impl/src/plumbing.rs:91:9
 132: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, (), ()>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/plumbing.rs:465:36
 133: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), ()>>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/plumbing.rs:400:44
 134: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::typeck_item_bodies, rustc_query_impl::plumbing::QueryCtxt>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/plumbing.rs:765:36
 135: <rustc_middle::ty::query::TyCtxtAt>::typeck_item_bodies
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/query.rs:254:17
 136: <rustc_middle::ty::context::TyCtxt>::typeck_item_bodies
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/query.rs:235:17
 137: rustc_typeck::check_crate::{closure#7}
             at /home/aaron/repos/rust/compiler/rustc_typeck/src/lib.rs:532:46
 138: <rustc_data_structures::profiling::VerboseTimingGuard>::run::<(), rustc_typeck::check_crate::{closure#7}>
             at /home/aaron/repos/rust/compiler/rustc_data_structures/src/profiling.rs:644:9
 139: <rustc_session::session::Session>::time::<(), rustc_typeck::check_crate::{closure#7}>
             at /home/aaron/repos/rust/compiler/rustc_session/src/utils.rs:16:9
 140: rustc_typeck::check_crate
             at /home/aaron/repos/rust/compiler/rustc_typeck/src/lib.rs:532:5
 141: rustc_interface::passes::analysis
             at /home/aaron/repos/rust/compiler/rustc_interface/src/passes.rs:926:5
 142: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl::<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:326:53
 143: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/dep_graph/mod.rs:55:46
 144: rustc_middle::ty::context::tls::enter_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1764:50
 145: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1748:9
 146: rustc_middle::ty::context::tls::enter_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1764:9
 147: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/dep_graph/mod.rs:55:13
 148: rustc_middle::ty::context::tls::with_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1792:40
 149: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1781:22
 150: rustc_middle::ty::context::tls::with_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1792:9
 151: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/dep_graph/mod.rs:52:9
 152: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl::<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:326:22
 153: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/dep_graph/graph.rs:274:13
 154: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/plumbing.rs:475:9
 155: stacker::maybe_grow::<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>
             at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.14/src/lib.rs:55:9
 156: rustc_data_structures::stack::ensure_sufficient_stack::<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>
             at /home/aaron/repos/rust/compiler/rustc_data_structures/src/stack.rs:16:5
 157: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>::{closure#0}::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_query_impl/src/plumbing.rs:103:17
 158: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>::{closure#0}::{closure#0}, (core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1764:50
 159: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>::{closure#0}::{closure#0}, (core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, (core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1748:9
 160: rustc_middle::ty::context::tls::enter_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>::{closure#0}::{closure#0}, (core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1764:9
 161: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_query_impl/src/plumbing.rs:102:13
 162: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>::{closure#0}, (core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1808:13
 163: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>::{closure#0}, (core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, (core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1792:40
 164: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>::{closure#0}, (core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, (core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, (core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1781:22
 165: rustc_middle::ty::context::tls::with_context::<rustc_middle::ty::context::tls::with_related_context<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>::{closure#0}, (core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex)>::{closure#0}, (core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1792:9
 166: rustc_middle::ty::context::tls::with_related_context::<<rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>::{closure#0}, (core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex)>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1805:9
 167: <rustc_query_impl::plumbing::QueryCtxt as rustc_query_system::query::QueryContext>::start_query::<(core::result::Result<(), rustc_errors::ErrorReported>, rustc_query_system::dep_graph::graph::DepNodeIndex), rustc_query_system::query::plumbing::execute_job<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>::{closure#3}>
             at /home/aaron/repos/rust/compiler/rustc_query_impl/src/plumbing.rs:91:9
 168: rustc_query_system::query::plumbing::execute_job::<rustc_query_impl::plumbing::QueryCtxt, (), core::result::Result<(), rustc_errors::ErrorReported>>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/plumbing.rs:465:36
 169: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), core::result::Result<(), rustc_errors::ErrorReported>>>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/plumbing.rs:400:44
 170: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
             at /home/aaron/repos/rust/compiler/rustc_query_system/src/query/plumbing.rs:765:36
 171: <rustc_middle::ty::query::TyCtxtAt>::analysis
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/query.rs:254:17
 172: <rustc_middle::ty::context::TyCtxt>::analysis
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/query.rs:235:17
 173: rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}
             at /home/aaron/repos/rust/compiler/rustc_driver/src/lib.rs:387:30
 174: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/passes.rs:821:42
 175: rustc_middle::ty::context::tls::enter_context::<<rustc_interface::passes::QueryContext>::enter<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1764:50
 176: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_interface::passes::QueryContext>::enter<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1748:9
 177: rustc_middle::ty::context::tls::enter_context::<<rustc_interface::passes::QueryContext>::enter<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
             at /home/aaron/repos/rust/compiler/rustc_middle/src/ty/context.rs:1764:9
 178: <rustc_interface::passes::QueryContext>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}::{closure#3}, core::result::Result<(), rustc_errors::ErrorReported>>
             at /home/aaron/repos/rust/compiler/rustc_interface/src/passes.rs:821:9
 179: rustc_driver::run_compiler::{closure#1}::{closure#2}
             at /home/aaron/repos/rust/compiler/rustc_driver/src/lib.rs:386:13
 180: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorReported>>
             at /home/aaron/repos/rust/compiler/rustc_interface/src/queries.rs:393:19
 181: rustc_driver::run_compiler::{closure#1}
             at /home/aaron/repos/rust/compiler/rustc_driver/src/lib.rs:315:22
 182: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#1}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/interface.rs:220:13
 183: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
             at /home/aaron/repos/rust/compiler/rustc_span/src/lib.rs:1011:5
 184: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>
             at /home/aaron/repos/rust/compiler/rustc_interface/src/interface.rs:214:5
 185: rustc_interface::interface::run_compiler::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/interface.rs:236:12
 186: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/util.rs:148:13
 187: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
             at /home/aaron/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
 188: rustc_span::create_session_globals_then::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}>
             at /home/aaron/repos/rust/compiler/rustc_span/src/lib.rs:109:5
 189: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}
             at /home/aaron/repos/rust/compiler/rustc_interface/src/util.rs:146:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -C incremental --crate-type lib

query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `CycleTwo: core::marker::Send`
#1 [typeck] type-checking `bar`
#2 [typeck_item_bodies] type-checking all item bodies
#3 [analysis] running analysis passes on this crate
end of query stack
