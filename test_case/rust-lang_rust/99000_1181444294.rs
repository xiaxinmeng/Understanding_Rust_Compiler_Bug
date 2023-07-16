plain
............................i.....ii.................................................... 1760/13174
........................................................................................ 1848/13174
...................................................................................i.... 1936/13174
........................................................................................ 2024/13174
.............................................................F.............F............ 2112/13174
........................................................................................ 2288/13174
........................................................................................ 2288/13174
.....................F.................F................................................ 2376/13174
........................................................................................ 2552/13174
........................................................................................ 2640/13174
........................................................................................ 2728/13174
........................................................................................ 2816/13174
---
failures:

---- [ui] src/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_middle/src/mir/interpret/queries.rs:76:13: did not expect inference variables here
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1391:9
stack backtrace:
   0:     0x7f1b2042179c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha87f97bb9c42f7a1
   0:     0x7f1b2042179c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha87f97bb9c42f7a1
   1:     0x7f1b204878b8 - core::fmt::write::h7cedeb0d86e686c1
   2:     0x7f1b20411dd1 - std::io::Write::write_fmt::hd1fcfe17c8b3e50a
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   3:     0x7f1b204247ae - std::panicking::default_hook::{{closure}}::hd548e6b2bb3eb711
   4:     0x7f1b20424476 - std::panicking::default_hook::h7741fd6664ab79e7
   5:     0x7f1b20dde794 - rustc_driver[f852af0d696e410b]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f1b20424f62 - std::panicking::rust_panic_with_hook::hc318c082bced4080
   7:     0x7f1b2372ada3 - std[c99ebb468f9d244]::panicking::begin_panic::<rustc_errors[782abd4b611703e6]::ExplicitBug>::{closure#0}
   8:     0x7f1b2372a076 - std[c99ebb468f9d244]::sys_common::backtrace::__rust_end_short_backtrace::<std[c99ebb468f9d244]::panicking::begin_panic<rustc_errors[782abd4b611703e6]::ExplicitBug>::{closure#0}, !>
   9:     0x7f1b20d733e6 - std[c99ebb468f9d244]::panicking::begin_panic::<rustc_errors[782abd4b611703e6]::ExplicitBug>
  10:     0x7f1b23888c76 - std[c99ebb468f9d244]::panic::panic_any::<rustc_errors[782abd4b611703e6]::ExplicitBug>
  11:     0x7f1b23887346 - <rustc_errors[782abd4b611703e6]::HandlerInner>::bug::<&alloc[29a2a132425083cc]::string::String>
  12:     0x7f1b23887000 - <rustc_errors[782abd4b611703e6]::Handler>::bug::<&alloc[29a2a132425083cc]::string::String>
  13:     0x7f1b23906ab5 - rustc_middle[e4acdad16398451c]::util::bug::opt_span_bug_fmt::<rustc_span[b3e4704f910b2b73]::span_encoding::Span>::{closure#0}
  14:     0x7f1b2390599b - rustc_middle[e4acdad16398451c]::ty::context::tls::with_opt::<rustc_middle[e4acdad16398451c]::util::bug::opt_span_bug_fmt<rustc_span[b3e4704f910b2b73]::span_encoding::Span>::{closure#0}, ()>::{closure#0}
  15:     0x7f1b23905946 - rustc_middle[e4acdad16398451c]::ty::context::tls::with_context_opt::<rustc_middle[e4acdad16398451c]::ty::context::tls::with_opt<rustc_middle[e4acdad16398451c]::util::bug::opt_span_bug_fmt<rustc_span[b3e4704f910b2b73]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  16:     0x7f1b239069f9 - rustc_middle[e4acdad16398451c]::util::bug::opt_span_bug_fmt::<rustc_span[b3e4704f910b2b73]::span_encoding::Span>
  17:     0x7f1b20d7b525 - rustc_middle[e4acdad16398451c]::util::bug::bug_fmt
  18:     0x7f1b23877b0b - <rustc_middle[e4acdad16398451c]::ty::context::TyCtxt>::const_eval_resolve_for_typeck
  19:     0x7f1b235f7658 - <rustc_infer[28c90180cdc353a6]::infer::InferCtxt>::const_eval_resolve
  20:     0x7f1b235e1ff9 - <rustc_infer[28c90180cdc353a6]::infer::InferCtxt>::try_const_eval_resolve
  21:     0x7f1b234393fb - <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::evaluate_predicate_recursively
  22:     0x7f1b23437f89 - <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::evaluate_predicates_recursively::<alloc[29a2a132425083cc]::vec::into_iter::IntoIter<rustc_infer[28c90180cdc353a6]::traits::Obligation<rustc_middle[e4acdad16398451c]::ty::Predicate>>>
  23:     0x7f1b233d4c6f - <rustc_infer[28c90180cdc353a6]::infer::InferCtxt>::probe::<core[992335a751240df9]::result::Result<rustc_middle[e4acdad16398451c]::traits::select::EvaluationResult, rustc_middle[e4acdad16398451c]::traits::select::OverflowError>, <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::evaluate_candidate::{closure#0}>::{closure#0}>
  24:     0x7f1b2343dee4 - <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::evaluate_candidate
  25:     0x7f1b234cd788 - <core[992335a751240df9]::iter::adapters::map::Map<core[992335a751240df9]::iter::adapters::map::Map<alloc[29a2a132425083cc]::vec::into_iter::IntoIter<rustc_middle[e4acdad16398451c]::traits::select::SelectionCandidate>, <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::candidate_from_obligation_no_cache::{closure#0}>, <core[992335a751240df9]::result::Result<core[992335a751240df9]::option::Option<rustc_trait_selection[a9542b9bf4f2282b]::traits::select::EvaluatedCandidate>, rustc_middle[e4acdad16398451c]::traits::SelectionError>>::transpose> as core[992335a751240df9]::iter::traits::iterator::Iterator>::try_fold::<(), <core[992335a751240df9]::iter::adapters::flatten::FlattenCompat<_, _> as core[992335a751240df9]::iter::traits::iterator::Iterator>::try_fold::flatten<core[992335a751240df9]::option::Option<core[992335a751240df9]::result::Result<rustc_trait_selection[a9542b9bf4f2282b]::traits::select::EvaluatedCandidate, rustc_middle[e4acdad16398451c]::traits::SelectionError>>, (), core[992335a751240df9]::ops::control_flow::ControlFlow<core[992335a751240df9]::ops::control_flow::ControlFlow<rustc_trait_selection[a9542b9bf4f2282b]::traits::select::EvaluatedCandidate>>, <core[992335a751240df9]::iter::adapters::GenericShunt<core[992335a751240df9]::iter::adapters::flatten::FlatMap<core[992335a751240df9]::iter::adapters::map::Map<alloc[29a2a132425083cc]::vec::into_iter::IntoIter<rustc_middle[e4acdad16398451c]::traits::select::SelectionCandidate>, <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::candidate_from_obligation_no_cache::{closure#0}>, core[992335a751240df9]::option::Option<core[992335a751240df9]::result::Result<rustc_trait_selection[a9542b9bf4f2282b]::traits::select::EvaluatedCandidate, rustc_middle[e4acdad16398451c]::traits::SelectionError>>, <core[992335a751240df9]::result::Result<core[992335a751240df9]::option::Option<rustc_trait_selection[a9542b9bf4f2282b]::traits::select::EvaluatedCandidate>, rustc_middle[e4acdad16398451c]::traits::SelectionError>>::transpose>, core[992335a751240df9]::result::Result<core[992335a751240df9]::convert::Infallible, rustc_middle[e4acdad16398451c]::traits::SelectionError>> as core[992335a751240df9]::iter::traits::iterator::Iterator>::try_fold<(), core[992335a751240df9]::iter::traits::iterator::Iterator::try_for_each::call<rustc_trait_selection[a9542b9bf4f2282b]::traits::select::EvaluatedCandidate, core[992335a751240df9]::ops::control_flow::ControlFlow<rustc_trait_selection[a9542b9bf4f2282b]::traits::select::EvaluatedCandidate>, core[992335a751240df9]::ops::control_flow::ControlFlow<rustc_trait_selection[a9542b9bf4f2282b]::traits::select::EvaluatedCandidate>::Break>::{closure#0}, core[992335a751240df9]::ops::control_flow::ControlFlow<rustc_trait_selection[a9542b9bf4f2282b]::traits::select::EvaluatedCandidate>>::{closure#0}>::{closure#0}, core[992335a751240df9]::ops::control_flow::ControlFlow<core[992335a751240df9]::ops::control_flow::ControlFlow<rustc_trait_selection[a9542b9bf4f2282b]::traits::select::EvaluatedCandidate>>>
  26:     0x7f1b23546279 - <core[992335a751240df9]::iter::adapters::GenericShunt<core[992335a751240df9]::iter::adapters::flatten::FlatMap<core[992335a751240df9]::iter::adapters::map::Map<alloc[29a2a132425083cc]::vec::into_iter::IntoIter<rustc_middle[e4acdad16398451c]::traits::select::SelectionCandidate>, <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::candidate_from_obligation_no_cache::{closure#0}>, core[992335a751240df9]::option::Option<core[992335a751240df9]::result::Result<rustc_trait_selection[a9542b9bf4f2282b]::traits::select::EvaluatedCandidate, rustc_middle[e4acdad16398451c]::traits::SelectionError>>, <core[992335a751240df9]::result::Result<core[992335a751240df9]::option::Option<rustc_trait_selection[a9542b9bf4f2282b]::traits::select::EvaluatedCandidate>, rustc_middle[e4acdad16398451c]::traits::SelectionError>>::transpose>, core[992335a751240df9]::result::Result<core[992335a751240df9]::convert::Infallible, rustc_middle[e4acdad16398451c]::traits::SelectionError>> as core[992335a751240df9]::iter::traits::iterator::Iterator>::next
  27:     0x7f1b23457bd7 - <alloc[29a2a132425083cc]::vec::Vec<rustc_trait_selection[a9542b9bf4f2282b]::traits::select::EvaluatedCandidate> as alloc[29a2a132425083cc]::vec::spec_from_iter::SpecFromIter<rustc_trait_selection[a9542b9bf4f2282b]::traits::select::EvaluatedCandidate, core[992335a751240df9]::iter::adapters::GenericShunt<core[992335a751240df9]::iter::adapters::flatten::FlatMap<core[992335a751240df9]::iter::adapters::map::Map<alloc[29a2a132425083cc]::vec::into_iter::IntoIter<rustc_middle[e4acdad16398451c]::traits::select::SelectionCandidate>, <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::candidate_from_obligation_no_cache::{closure#0}>, core[992335a751240df9]::option::Option<core[992335a751240df9]::result::Result<rustc_trait_selection[a9542b9bf4f2282b]::traits::select::EvaluatedCandidate, rustc_middle[e4acdad16398451c]::traits::SelectionError>>, <core[992335a751240df9]::result::Result<core[992335a751240df9]::option::Option<rustc_trait_selection[a9542b9bf4f2282b]::traits::select::EvaluatedCandidate>, rustc_middle[e4acdad16398451c]::traits::SelectionError>>::transpose>, core[992335a751240df9]::result::Result<core[992335a751240df9]::convert::Infallible, rustc_middle[e4acdad16398451c]::traits::SelectionError>>>>::from_iter
  28:     0x7f1b2353815d - core[992335a751240df9]::iter::adapters::try_process::<core[992335a751240df9]::iter::adapters::flatten::FlatMap<core[992335a751240df9]::iter::adapters::map::Map<alloc[29a2a132425083cc]::vec::into_iter::IntoIter<rustc_middle[e4acdad16398451c]::traits::select::SelectionCandidate>, <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::candidate_from_obligation_no_cache::{closure#0}>, core[992335a751240df9]::option::Option<core[992335a751240df9]::result::Result<rustc_trait_selection[a9542b9bf4f2282b]::traits::select::EvaluatedCandidate, rustc_middle[e4acdad16398451c]::traits::SelectionError>>, <core[992335a751240df9]::result::Result<core[992335a751240df9]::option::Option<rustc_trait_selection[a9542b9bf4f2282b]::traits::select::EvaluatedCandidate>, rustc_middle[e4acdad16398451c]::traits::SelectionError>>::transpose>, rustc_trait_selection[a9542b9bf4f2282b]::traits::select::EvaluatedCandidate, core[992335a751240df9]::result::Result<core[992335a751240df9]::convert::Infallible, rustc_middle[e4acdad16398451c]::traits::SelectionError>, <core[992335a751240df9]::result::Result<alloc[29a2a132425083cc]::vec::Vec<rustc_trait_selection[a9542b9bf4f2282b]::traits::select::EvaluatedCandidate>, rustc_middle[e4acdad16398451c]::traits::SelectionError> as core[992335a751240df9]::iter::traits::collect::FromIterator<core[992335a751240df9]::result::Result<rustc_trait_selection[a9542b9bf4f2282b]::traits::select::EvaluatedCandidate, rustc_middle[e4acdad16398451c]::traits::SelectionError>>>::from_iter<core[992335a751240df9]::iter::adapters::flatten::FlatMap<core[992335a751240df9]::iter::adapters::map::Map<alloc[29a2a132425083cc]::vec::into_iter::IntoIter<rustc_middle[e4acdad16398451c]::traits::select::SelectionCandidate>, <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::candidate_from_obligation_no_cache::{closure#0}>, core[992335a751240df9]::option::Option<core[992335a751240df9]::result::Result<rustc_trait_selection[a9542b9bf4f2282b]::traits::select::EvaluatedCandidate, rustc_middle[e4acdad16398451c]::traits::SelectionError>>, <core[992335a751240df9]::result::Result<core[992335a751240df9]::option::Option<rustc_trait_selection[a9542b9bf4f2282b]::traits::select::EvaluatedCandidate>, rustc_middle[e4acdad16398451c]::traits::SelectionError>>::transpose>>::{closure#0}, alloc[29a2a132425083cc]::vec::Vec<rustc_trait_selection[a9542b9bf4f2282b]::traits::select::EvaluatedCandidate>>
  29:     0x7f1b23417455 - <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::candidate_from_obligation_no_cache
  30:     0x7f1b234c42cd - <rustc_query_system[bac8b8e6cbe04b6]::dep_graph::graph::DepGraph<rustc_middle[e4acdad16398451c]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[e4acdad16398451c]::ty::context::TyCtxt, <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::candidate_from_obligation::{closure#0}, core[992335a751240df9]::result::Result<core[992335a751240df9]::option::Option<rustc_middle[e4acdad16398451c]::traits::select::SelectionCandidate>, rustc_middle[e4acdad16398451c]::traits::SelectionError>>::{closure#0}, core[992335a751240df9]::result::Result<core[992335a751240df9]::option::Option<rustc_middle[e4acdad16398451c]::traits::select::SelectionCandidate>, rustc_middle[e4acdad16398451c]::traits::SelectionError>>
  31:     0x7f1b234217d8 - <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::candidate_from_obligation
  32:     0x7f1b2341c836 - <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::select_from_obligation
  33:     0x7f1b23436cb7 - <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::select
  34:     0x7f1b234f0cf2 - <rustc_trait_selection[a9542b9bf4f2282b]::traits::fulfill::FulfillProcessor>::process_trait_obligation
  35:     0x7f1b234ddbe5 - <rustc_trait_selection[a9542b9bf4f2282b]::traits::fulfill::FulfillProcessor as rustc_data_structures[196d74b7f753b39b]::obligation_forest::ObligationProcessor>::process_obligation
  36:     0x7f1b2353489b - <rustc_data_structures[196d74b7f753b39b]::obligation_forest::ObligationForest<rustc_trait_selection[a9542b9bf4f2282b]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[a9542b9bf4f2282b]::traits::fulfill::FulfillProcessor, rustc_data_structures[196d74b7f753b39b]::obligation_forest::Outcome<rustc_trait_selection[a9542b9bf4f2282b]::traits::fulfill::PendingPredicateObligation, rustc_infer[28c90180cdc353a6]::traits::FulfillmentErrorCode>>
  37:     0x7f1b234dcbba - <rustc_trait_selection[a9542b9bf4f2282b]::traits::fulfill::FulfillmentContext as rustc_infer[28c90180cdc353a6]::traits::engine::TraitEngine>::select_where_possible
  38:     0x7f1b21705571 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::resolve_vars_with_obligations
  39:     0x7f1b2170a8ca - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::structurally_resolved_type
  40:     0x7f1b216dd060 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_call
  41:     0x7f1b2175e0a9 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_expr_kind
  42:     0x7f1b216f4ee0 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  43:     0x7f1b2175ce49 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  44:     0x7f1b21763476 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_expr_kind
  45:     0x7f1b216f4ee0 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  46:     0x7f1b2175ce49 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  47:     0x7f1b21713afe - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_stmt
  48:     0x7f1b21714084 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  49:     0x7f1b2175e341 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_expr_kind
  50:     0x7f1b216f4ee0 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  51:     0x7f1b2175ce49 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  52:     0x7f1b216f67c9 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_return_expr
  53:     0x7f1b21964399 - rustc_typeck[956e19107821422a]::check::check::check_fn
  54:     0x7f1b2191830a - <rustc_infer[28c90180cdc353a6]::infer::InferCtxtBuilder>::enter::<&rustc_middle[e4acdad16398451c]::ty::context::TypeckResults, <rustc_typeck[956e19107821422a]::check::inherited::InheritedBuilder>::enter<rustc_typeck[956e19107821422a]::check::typeck_with_fallback<rustc_typeck[956e19107821422a]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[e4acdad16398451c]::ty::context::TypeckResults>::{closure#0}>
  55:     0x7f1b2183559e - rustc_typeck[956e19107821422a]::check::typeck
  56:     0x7f1b2299d83d - rustc_query_system[bac8b8e6cbe04b6]::query::plumbing::try_execute_query::<rustc_query_impl[1637cc171863d90f]::plumbing::QueryCtxt, rustc_query_system[bac8b8e6cbe04b6]::query::caches::DefaultCache<rustc_span[b3e4704f910b2b73]::def_id::LocalDefId, &rustc_middle[e4acdad16398451c]::ty::context::TypeckResults>>
  57:     0x7f1b22abdfd7 - rustc_query_system[bac8b8e6cbe04b6]::query::plumbing::get_query::<rustc_query_impl[1637cc171863d90f]::queries::typeck, rustc_query_impl[1637cc171863d90f]::plumbing::QueryCtxt>
  58:     0x7f1b225afdf4 - <rustc_query_impl[1637cc171863d90f]::Queries as rustc_middle[e4acdad16398451c]::ty::query::QueryEngine>::typeck
  59:     0x7f1b217d3065 - rustc_typeck[956e19107821422a]::collect::type_of::opt_const_param_of
  60:     0x7f1b2298efa5 - rustc_query_system[bac8b8e6cbe04b6]::query::plumbing::try_execute_query::<rustc_query_impl[1637cc171863d90f]::plumbing::QueryCtxt, rustc_query_system[bac8b8e6cbe04b6]::query::caches::DefaultCache<rustc_span[b3e4704f910b2b73]::def_id::LocalDefId, core[992335a751240df9]::option::Option<rustc_span[b3e4704f910b2b73]::def_id::DefId>>>
  61:     0x7f1b22a83a0e - rustc_query_system[bac8b8e6cbe04b6]::query::plumbing::get_query::<rustc_query_impl[1637cc171863d90f]::queries::opt_const_param_of, rustc_query_impl[1637cc171863d90f]::plumbing::QueryCtxt>
  62:     0x7f1b22592474 - <rustc_query_impl[1637cc171863d90f]::Queries as rustc_middle[e4acdad16398451c]::ty::query::QueryEngine>::opt_const_param_of
  63:     0x7f1b21834ed1 - rustc_typeck[956e19107821422a]::check::typeck
  64:     0x7f1b2299d83d - rustc_query_system[bac8b8e6cbe04b6]::query::plumbing::try_execute_query::<rustc_query_impl[1637cc171863d90f]::plumbing::QueryCtxt, rustc_query_system[bac8b8e6cbe04b6]::query::caches::DefaultCache<rustc_span[b3e4704f910b2b73]::def_id::LocalDefId, &rustc_middle[e4acdad16398451c]::ty::context::TypeckResults>>
  65:     0x7f1b22abdfd7 - rustc_query_system[bac8b8e6cbe04b6]::query::plumbing::get_query::<rustc_query_impl[1637cc171863d90f]::queries::typeck, rustc_query_impl[1637cc171863d90f]::plumbing::QueryCtxt>
  66:     0x7f1b225afdf4 - <rustc_query_impl[1637cc171863d90f]::Queries as rustc_middle[e4acdad16398451c]::ty::query::QueryEngine>::typeck
  67:     0x7f1b219f7fca - <rustc_middle[e4acdad16398451c]::hir::map::Map>::par_body_owners::<rustc_typeck[956e19107821422a]::check::typeck_item_bodies::{closure#0}>
  68:     0x7f1b2183a9bd - rustc_typeck[956e19107821422a]::check::typeck_item_bodies
  69:     0x7f1b229e3cbf - rustc_query_system[bac8b8e6cbe04b6]::query::plumbing::try_execute_query::<rustc_query_impl[1637cc171863d90f]::plumbing::QueryCtxt, rustc_query_system[bac8b8e6cbe04b6]::query::caches::DefaultCache<(), ()>>
  70:     0x7f1b22a83b35 - rustc_query_system[bac8b8e6cbe04b6]::query::plumbing::get_query::<rustc_query_impl[1637cc171863d90f]::queries::typeck_item_bodies, rustc_query_impl[1637cc171863d90f]::plumbing::QueryCtxt>
  71:     0x7f1b225af89e - <rustc_query_impl[1637cc171863d90f]::Queries as rustc_middle[e4acdad16398451c]::ty::query::QueryEngine>::typeck_item_bodies
  72:     0x7f1b218564ca - <rustc_session[c3b8890bf519571]::session::Session>::time::<(), rustc_typeck[956e19107821422a]::check_crate::{closure#7}>
  73:     0x7f1b21a2b5ae - rustc_typeck[956e19107821422a]::check_crate
  74:     0x7f1b20ef43a1 - rustc_interface[71f375b04d6f2c83]::passes::analysis
  75:     0x7f1b229d8a40 - rustc_query_system[bac8b8e6cbe04b6]::query::plumbing::try_execute_query::<rustc_query_impl[1637cc171863d90f]::plumbing::QueryCtxt, rustc_query_system[bac8b8e6cbe04b6]::query::caches::DefaultCache<(), core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>>>
  76:     0x7f1b22abe3b2 - rustc_query_system[bac8b8e6cbe04b6]::query::plumbing::get_query::<rustc_query_impl[1637cc171863d90f]::queries::analysis, rustc_query_impl[1637cc171863d90f]::plumbing::QueryCtxt>
  77:     0x7f1b225934ae - <rustc_query_impl[1637cc171863d90f]::Queries as rustc_middle[e4acdad16398451c]::ty::query::QueryEngine>::analysis
  78:     0x7f1b20e34925 - <rustc_interface[71f375b04d6f2c83]::passes::QueryContext>::enter::<rustc_driver[f852af0d696e410b]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>>
  79:     0x7f1b20de5e5c - <rustc_interface[71f375b04d6f2c83]::interface::Compiler>::enter::<rustc_driver[f852af0d696e410b]::run_compiler::{closure#1}::{closure#2}, core[992335a751240df9]::result::Result<core[992335a751240df9]::option::Option<rustc_interface[71f375b04d6f2c83]::queries::Linker>, rustc_errors[782abd4b611703e6]::ErrorGuaranteed>>
  80:     0x7f1b20dcde48 - rustc_span[b3e4704f910b2b73]::with_source_map::<core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>, rustc_interface[71f375b04d6f2c83]::interface::create_compiler_and_run<core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>, rustc_driver[f852af0d696e410b]::run_compiler::{closure#1}>::{closure#1}>
  81:     0x7f1b20de7d29 - rustc_interface[71f375b04d6f2c83]::interface::create_compiler_and_run::<core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>, rustc_driver[f852af0d696e410b]::run_compiler::{closure#1}>
  82:     0x7f1b20dc7e5f - <scoped_tls[e65d73f394969a0b]::ScopedKey<rustc_span[b3e4704f910b2b73]::SessionGlobals>>::set::<rustc_interface[71f375b04d6f2c83]::interface::run_compiler<core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>, rustc_driver[f852af0d696e410b]::run_compiler::{closure#1}>::{closure#0}, core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>>
  83:     0x7f1b20dd1ba9 - std[c99ebb468f9d244]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[71f375b04d6f2c83]::util::run_in_thread_pool_with_globals<rustc_interface[71f375b04d6f2c83]::interface::run_compiler<core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>, rustc_driver[f852af0d696e410b]::run_compiler::{closure#1}>::{closure#0}, core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>>::{closure#0}, core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>>
  84:     0x7f1b20e3bc79 - <<std[c99ebb468f9d244]::thread::Builder>::spawn_unchecked_<rustc_interface[71f375b04d6f2c83]::util::run_in_thread_pool_with_globals<rustc_interface[71f375b04d6f2c83]::interface::run_compiler<core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>, rustc_driver[f852af0d696e410b]::run_compiler::{closure#1}>::{closure#0}, core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>>::{closure#0}, core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>>::{closure#1} as core[992335a751240df9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  85:     0x7f1b20430303 - std::sys::unix::thread::Thread::new::thread_start::h66a50617e17509ef
  86:     0x7f1b1a97c609 - start_thread
  87:     0x7f1b2028f133 - clone
  88:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (b61ef4fde 2022-07-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [opt_const_param_of] computing the optional const parameter of `main::{constant#0}`
#2 [typeck] type-checking `main::{constant#0}`
#3 [typeck_item_bodies] type-checking all item bodies
#4 [analysis] running analysis passes on this crate
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/infer-too-generic.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/infer-too-generic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/infer-too-generic/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/infer-too-generic/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_middle/src/mir/interpret/queries.rs:76:13: did not expect inference variables here
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1391:9
stack backtrace:
   0:     0x7fe52b5f379c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha87f97bb9c42f7a1
   0:     0x7fe52b5f379c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha87f97bb9c42f7a1
   1:     0x7fe52b6598b8 - core::fmt::write::h7cedeb0d86e686c1
   2:     0x7fe52b5e3dd1 - std::io::Write::write_fmt::hd1fcfe17c8b3e50a
   3:     0x7fe52b5f67ae - std::panicking::default_hook::{{closure}}::hd548e6b2bb3eb711
   4:     0x7fe52b5f6476 - std::panicking::default_hook::h7741fd6664ab79e7
   5:     0x7fe52bfb0794 - rustc_driver[f852af0d696e410b]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fe52b5f6f62 - std::panicking::rust_panic_with_hook::hc318c082bced4080
   7:     0x7fe52e8fcda3 - std[c99ebb468f9d244]::panicking::begin_panic::<rustc_errors[782abd4b611703e6]::ExplicitBug>::{closure#0}
   8:     0x7fe52e8fc076 - std[c99ebb468f9d244]::sys_common::backtrace::__rust_end_short_backtrace::<std[c99ebb468f9d244]::panicking::begin_panic<rustc_errors[782abd4b611703e6]::ExplicitBug>::{closure#0}, !>
   9:     0x7fe52bf453e6 - std[c99ebb468f9d244]::panicking::begin_panic::<rustc_errors[782abd4b611703e6]::ExplicitBug>
  10:     0x7fe52ea5ac76 - std[c99ebb468f9d244]::panic::panic_any::<rustc_errors[782abd4b611703e6]::ExplicitBug>
  11:     0x7fe52ea59346 - <rustc_errors[782abd4b611703e6]::HandlerInner>::bug::<&alloc[29a2a132425083cc]::string::String>
  12:     0x7fe52ea59000 - <rustc_errors[782abd4b611703e6]::Handler>::bug::<&alloc[29a2a132425083cc]::string::String>
  13:     0x7fe52ead8ab5 - rustc_middle[e4acdad16398451c]::util::bug::opt_span_bug_fmt::<rustc_span[b3e4704f910b2b73]::span_encoding::Span>::{closure#0}
  14:     0x7fe52ead799b - rustc_middle[e4acdad16398451c]::ty::context::tls::with_opt::<rustc_middle[e4acdad16398451c]::util::bug::opt_span_bug_fmt<rustc_span[b3e4704f910b2b73]::span_encoding::Span>::{closure#0}, ()>::{closure#0}
  15:     0x7fe52ead7946 - rustc_middle[e4acdad16398451c]::ty::context::tls::with_context_opt::<rustc_middle[e4acdad16398451c]::ty::context::tls::with_opt<rustc_middle[e4acdad16398451c]::util::bug::opt_span_bug_fmt<rustc_span[b3e4704f910b2b73]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  16:     0x7fe52ead89f9 - rustc_middle[e4acdad16398451c]::util::bug::opt_span_bug_fmt::<rustc_span[b3e4704f910b2b73]::span_encoding::Span>
  17:     0x7fe52bf4d525 - rustc_middle[e4acdad16398451c]::util::bug::bug_fmt
  18:     0x7fe52ea49b0b - <rustc_middle[e4acdad16398451c]::ty::context::TyCtxt>::const_eval_resolve_for_typeck
  19:     0x7fe52e7c9658 - <rustc_infer[28c90180cdc353a6]::infer::InferCtxt>::const_eval_resolve
  20:     0x7fe52e6815d6 - rustc_trait_selection[a9542b9bf4f2282b]::traits::const_evaluatable::is_const_evaluatable
  21:     0x7fe52e6afd37 - <rustc_trait_selection[a9542b9bf4f2282b]::traits::fulfill::FulfillProcessor as rustc_data_structures[196d74b7f753b39b]::obligation_forest::ObligationProcessor>::process_obligation
  22:     0x7fe52e70689b - <rustc_data_structures[196d74b7f753b39b]::obligation_forest::ObligationForest<rustc_trait_selection[a9542b9bf4f2282b]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[a9542b9bf4f2282b]::traits::fulfill::FulfillProcessor, rustc_data_structures[196d74b7f753b39b]::obligation_forest::Outcome<rustc_trait_selection[a9542b9bf4f2282b]::traits::fulfill::PendingPredicateObligation, rustc_infer[28c90180cdc353a6]::traits::FulfillmentErrorCode>>
  23:     0x7fe52e6aebba - <rustc_trait_selection[a9542b9bf4f2282b]::traits::fulfill::FulfillmentContext as rustc_infer[28c90180cdc353a6]::traits::engine::TraitEngine>::select_where_possible
  24:     0x7fe52c8de08d - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_argument_types
  25:     0x7fe52c8b2088 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::confirm_builtin_call
  26:     0x7fe52c8afb3b - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_call
  27:     0x7fe52c9300a9 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_expr_kind
  28:     0x7fe52c8c6ee0 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  29:     0x7fe52c92ee49 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  30:     0x7fe52c8e5785 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_decl
  31:     0x7fe52c8e598a - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_stmt
  32:     0x7fe52c8e6084 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  33:     0x7fe52c930341 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_expr_kind
  34:     0x7fe52c8c6ee0 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  35:     0x7fe52c92ee49 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  36:     0x7fe52c8c87c9 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_return_expr
  37:     0x7fe52cb36399 - rustc_typeck[956e19107821422a]::check::check::check_fn
  38:     0x7fe52caea30a - <rustc_infer[28c90180cdc353a6]::infer::InferCtxtBuilder>::enter::<&rustc_middle[e4acdad16398451c]::ty::context::TypeckResults, <rustc_typeck[956e19107821422a]::check::inherited::InheritedBuilder>::enter<rustc_typeck[956e19107821422a]::check::typeck_with_fallback<rustc_typeck[956e19107821422a]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[e4acdad16398451c]::ty::context::TypeckResults>::{closure#0}>
  39:     0x7fe52ca0759e - rustc_typeck[956e19107821422a]::check::typeck
  40:     0x7fe52db6f83d - rustc_query_system[bac8b8e6cbe04b6]::query::plumbing::try_execute_query::<rustc_query_impl[1637cc171863d90f]::plumbing::QueryCtxt, rustc_query_system[bac8b8e6cbe04b6]::query::caches::DefaultCache<rustc_span[b3e4704f910b2b73]::def_id::LocalDefId, &rustc_middle[e4acdad16398451c]::ty::context::TypeckResults>>
  41:     0x7fe52dc8ffd7 - rustc_query_system[bac8b8e6cbe04b6]::query::plumbing::get_query::<rustc_query_impl[1637cc171863d90f]::queries::typeck, rustc_query_impl[1637cc171863d90f]::plumbing::QueryCtxt>
  42:     0x7fe52d781df4 - <rustc_query_impl[1637cc171863d90f]::Queries as rustc_middle[e4acdad16398451c]::ty::query::QueryEngine>::typeck
  43:     0x7fe52cbc9fca - <rustc_middle[e4acdad16398451c]::hir::map::Map>::par_body_owners::<rustc_typeck[956e19107821422a]::check::typeck_item_bodies::{closure#0}>
  44:     0x7fe52ca0c9bd - rustc_typeck[956e19107821422a]::check::typeck_item_bodies
  45:     0x7fe52dbb5cbf - rustc_query_system[bac8b8e6cbe04b6]::query::plumbing::try_execute_query::<rustc_query_impl[1637cc171863d90f]::plumbing::QueryCtxt, rustc_query_system[bac8b8e6cbe04b6]::query::caches::DefaultCache<(), ()>>
  46:     0x7fe52dc55b35 - rustc_query_system[bac8b8e6cbe04b6]::query::plumbing::get_query::<rustc_query_impl[1637cc171863d90f]::queries::typeck_item_bodies, rustc_query_impl[1637cc171863d90f]::plumbing::QueryCtxt>
  47:     0x7fe52d78189e - <rustc_query_impl[1637cc171863d90f]::Queries as rustc_middle[e4acdad16398451c]::ty::query::QueryEngine>::typeck_item_bodies
  48:     0x7fe52ca284ca - <rustc_session[c3b8890bf519571]::session::Session>::time::<(), rustc_typeck[956e19107821422a]::check_crate::{closure#7}>
  49:     0x7fe52cbfd5ae - rustc_typeck[956e19107821422a]::check_crate
  50:     0x7fe52c0c63a1 - rustc_interface[71f375b04d6f2c83]::passes::analysis
  51:     0x7fe52dbaaa40 - rustc_query_system[bac8b8e6cbe04b6]::query::plumbing::try_execute_query::<rustc_query_impl[1637cc171863d90f]::plumbing::QueryCtxt, rustc_query_system[bac8b8e6cbe04b6]::query::caches::DefaultCache<(), core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>>>
  52:     0x7fe52dc903b2 - rustc_query_system[bac8b8e6cbe04b6]::query::plumbing::get_query::<rustc_query_impl[1637cc171863d90f]::queries::analysis, rustc_query_impl[1637cc171863d90f]::plumbing::QueryCtxt>
  53:     0x7fe52d7654ae - <rustc_query_impl[1637cc171863d90f]::Queries as rustc_middle[e4acdad16398451c]::ty::query::QueryEngine>::analysis
  54:     0x7fe52c006925 - <rustc_interface[71f375b04d6f2c83]::passes::QueryContext>::enter::<rustc_driver[f852af0d696e410b]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>>
  55:     0x7fe52bfb7e5c - <rustc_interface[71f375b04d6f2c83]::interface::Compiler>::enter::<rustc_driver[f852af0d696e410b]::run_compiler::{closure#1}::{closure#2}, core[992335a751240df9]::result::Result<core[992335a751240df9]::option::Option<rustc_interface[71f375b04d6f2c83]::queries::Linker>, rustc_errors[782abd4b611703e6]::ErrorGuaranteed>>
  56:     0x7fe52bf9fe48 - rustc_span[b3e4704f910b2b73]::with_source_map::<core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>, rustc_interface[71f375b04d6f2c83]::interface::create_compiler_and_run<core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>, rustc_driver[f852af0d696e410b]::run_compiler::{closure#1}>::{closure#1}>
  57:     0x7fe52bfb9d29 - rustc_interface[71f375b04d6f2c83]::interface::create_compiler_and_run::<core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>, rustc_driver[f852af0d696e410b]::run_compiler::{closure#1}>
  58:     0x7fe52bf99e5f - <scoped_tls[e65d73f394969a0b]::ScopedKey<rustc_span[b3e4704f910b2b73]::SessionGlobals>>::set::<rustc_interface[71f375b04d6f2c83]::interface::run_compiler<core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>, rustc_driver[f852af0d696e410b]::run_compiler::{closure#1}>::{closure#0}, core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>>
  59:     0x7fe52bfa3ba9 - std[c99ebb468f9d244]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[71f375b04d6f2c83]::util::run_in_thread_pool_with_globals<rustc_interface[71f375b04d6f2c83]::interface::run_compiler<core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>, rustc_driver[f852af0d696e410b]::run_compiler::{closure#1}>::{closure#0}, core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>>::{closure#0}, core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>>
  60:     0x7fe52c00dc79 - <<std[c99ebb468f9d244]::thread::Builder>::spawn_unchecked_<rustc_interface[71f375b04d6f2c83]::util::run_in_thread_pool_with_globals<rustc_interface[71f375b04d6f2c83]::interface::run_compiler<core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>, rustc_driver[f852af0d696e410b]::run_compiler::{closure#1}>::{closure#0}, core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>>::{closure#0}, core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>>::{closure#1} as core[992335a751240df9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  61:     0x7fe52b602303 - std::sys::unix::thread::Thread::new::thread_start::h66a50617e17509ef
  62:     0x7fe525b4e609 - start_thread
  63:     0x7fe52b461133 - clone
  64:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (b61ef4fde 2022-07-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/const-generics/overlapping_impls.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/overlapping_impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/overlapping_impls" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/overlapping_impls/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0119]: conflicting implementations of trait `Bar` for type `Foo2<{ Protocol::Variant1 }, _>`
   |
   |
LL | impl<T> Bar for Foo2<{ Protocol::Variant1 }, T> {}
   | ----------------------------------------------- first implementation here
LL | impl<T> Bar for Foo2<{ Protocol::Variant2 }, T> {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `Foo2<{ Protocol::Variant1 }, _>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0119`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/const-generics/occurs-check/unused-substs-5.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/occurs-check/unused-substs-5.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/occurs-check/unused-substs-5" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/occurs-check/unused-substs-5/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_middle/src/mir/interpret/queries.rs:76:13: did not expect inference variables here
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1391:9
stack backtrace:
   0:     0x7f1267ed079c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha87f97bb9c42f7a1
   0:     0x7f1267ed079c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha87f97bb9c42f7a1
   1:     0x7f1267f368b8 - core::fmt::write::h7cedeb0d86e686c1
   2:     0x7f1267ec0dd1 - std::io::Write::write_fmt::hd1fcfe17c8b3e50a
   3:     0x7f1267ed37ae - std::panicking::default_hook::{{closure}}::hd548e6b2bb3eb711
   4:     0x7f1267ed3476 - std::panicking::default_hook::h7741fd6664ab79e7
   5:     0x7f126888d794 - rustc_driver[f852af0d696e410b]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f1267ed3f62 - std::panicking::rust_panic_with_hook::hc318c082bced4080
   7:     0x7f126b1d9da3 - std[c99ebb468f9d244]::panicking::begin_panic::<rustc_errors[782abd4b611703e6]::ExplicitBug>::{closure#0}
   8:     0x7f126b1d9076 - std[c99ebb468f9d244]::sys_common::backtrace::__rust_end_short_backtrace::<std[c99ebb468f9d244]::panicking::begin_panic<rustc_errors[782abd4b611703e6]::ExplicitBug>::{closure#0}, !>
   9:     0x7f12688223e6 - std[c99ebb468f9d244]::panicking::begin_panic::<rustc_errors[782abd4b611703e6]::ExplicitBug>
  10:     0x7f126b337c76 - std[c99ebb468f9d244]::panic::panic_any::<rustc_errors[782abd4b611703e6]::ExplicitBug>
  11:     0x7f126b336346 - <rustc_errors[782abd4b611703e6]::HandlerInner>::bug::<&alloc[29a2a132425083cc]::string::String>
  12:     0x7f126b336000 - <rustc_errors[782abd4b611703e6]::Handler>::bug::<&alloc[29a2a132425083cc]::string::String>
  13:     0x7f126b3b5ab5 - rustc_middle[e4acdad16398451c]::util::bug::opt_span_bug_fmt::<rustc_span[b3e4704f910b2b73]::span_encoding::Span>::{closure#0}
  14:     0x7f126b3b499b - rustc_middle[e4acdad16398451c]::ty::context::tls::with_opt::<rustc_middle[e4acdad16398451c]::util::bug::opt_span_bug_fmt<rustc_span[b3e4704f910b2b73]::span_encoding::Span>::{closure#0}, ()>::{closure#0}
  15:     0x7f126b3b4946 - rustc_middle[e4acdad16398451c]::ty::context::tls::with_context_opt::<rustc_middle[e4acdad16398451c]::ty::context::tls::with_opt<rustc_middle[e4acdad16398451c]::util::bug::opt_span_bug_fmt<rustc_span[b3e4704f910b2b73]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  16:     0x7f126b3b59f9 - rustc_middle[e4acdad16398451c]::util::bug::opt_span_bug_fmt::<rustc_span[b3e4704f910b2b73]::span_encoding::Span>
  17:     0x7f126882a525 - rustc_middle[e4acdad16398451c]::util::bug::bug_fmt
  18:     0x7f126b326b0b - <rustc_middle[e4acdad16398451c]::ty::context::TyCtxt>::const_eval_resolve_for_typeck
  19:     0x7f126b0a6658 - <rustc_infer[28c90180cdc353a6]::infer::InferCtxt>::const_eval_resolve
  20:     0x7f126b090ff9 - <rustc_infer[28c90180cdc353a6]::infer::InferCtxt>::try_const_eval_resolve
  21:     0x7f126aee83fb - <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::evaluate_predicate_recursively
  22:     0x7f126aee68b9 - <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::evaluate_predicates_recursively::<alloc[29a2a132425083cc]::vec::Vec<rustc_infer[28c90180cdc353a6]::traits::Obligation<rustc_middle[e4acdad16398451c]::ty::Predicate>>>
  23:     0x7f126ae84338 - <rustc_infer[28c90180cdc353a6]::infer::InferCtxt>::probe::<core[992335a751240df9]::result::Result<rustc_middle[e4acdad16398451c]::traits::select::EvaluationResult, rustc_middle[e4acdad16398451c]::traits::select::OverflowError>, <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::where_clause_may_apply::{closure#0}>::{closure#0}>
  24:     0x7f126aed3bac - <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::assemble_candidates
  25:     0x7f126aec51c8 - <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::candidate_from_obligation_no_cache
  26:     0x7f126af732cd - <rustc_query_system[bac8b8e6cbe04b6]::dep_graph::graph::DepGraph<rustc_middle[e4acdad16398451c]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[e4acdad16398451c]::ty::context::TyCtxt, <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::candidate_from_obligation::{closure#0}, core[992335a751240df9]::result::Result<core[992335a751240df9]::option::Option<rustc_middle[e4acdad16398451c]::traits::select::SelectionCandidate>, rustc_middle[e4acdad16398451c]::traits::SelectionError>>::{closure#0}, core[992335a751240df9]::result::Result<core[992335a751240df9]::option::Option<rustc_middle[e4acdad16398451c]::traits::select::SelectionCandidate>, rustc_middle[e4acdad16398451c]::traits::SelectionError>>
  27:     0x7f126aed07d8 - <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::candidate_from_obligation
  28:     0x7f126aecc0c2 - <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::evaluate_stack
  29:     0x7f126af72dcd - <rustc_query_system[bac8b8e6cbe04b6]::dep_graph::graph::DepGraph<rustc_middle[e4acdad16398451c]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[e4acdad16398451c]::ty::context::TyCtxt, <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively::{closure#2}, core[992335a751240df9]::result::Result<rustc_middle[e4acdad16398451c]::traits::select::EvaluationResult, rustc_middle[e4acdad16398451c]::traits::select::OverflowError>>::{closure#0}, core[992335a751240df9]::result::Result<rustc_middle[e4acdad16398451c]::traits::select::EvaluationResult, rustc_middle[e4acdad16398451c]::traits::select::OverflowError>>
  30:     0x7f126aee9d71 - <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively
  31:     0x7f126aee77b4 - <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::evaluate_predicate_recursively
  32:     0x7f126aee6f89 - <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::evaluate_predicates_recursively::<alloc[29a2a132425083cc]::vec::into_iter::IntoIter<rustc_infer[28c90180cdc353a6]::traits::Obligation<rustc_middle[e4acdad16398451c]::ty::Predicate>>>
  33:     0x7f126ae83c6f - <rustc_infer[28c90180cdc353a6]::infer::InferCtxt>::probe::<core[992335a751240df9]::result::Result<rustc_middle[e4acdad16398451c]::traits::select::EvaluationResult, rustc_middle[e4acdad16398451c]::traits::select::OverflowError>, <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::evaluate_candidate::{closure#0}>::{closure#0}>
  34:     0x7f126aeecee4 - <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::evaluate_candidate
  35:     0x7f126aecc154 - <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::evaluate_stack
  36:     0x7f126af72dcd - <rustc_query_system[bac8b8e6cbe04b6]::dep_graph::graph::DepGraph<rustc_middle[e4acdad16398451c]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[e4acdad16398451c]::ty::context::TyCtxt, <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively::{closure#2}, core[992335a751240df9]::result::Result<rustc_middle[e4acdad16398451c]::traits::select::EvaluationResult, rustc_middle[e4acdad16398451c]::traits::select::OverflowError>>::{closure#0}, core[992335a751240df9]::result::Result<rustc_middle[e4acdad16398451c]::traits::select::EvaluationResult, rustc_middle[e4acdad16398451c]::traits::select::OverflowError>>
  37:     0x7f126aee9d71 - <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::evaluate_trait_predicate_recursively
  38:     0x7f126aee77b4 - <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::evaluate_predicate_recursively
  39:     0x7f126ae8478c - <rustc_infer[28c90180cdc353a6]::infer::InferCtxt>::probe::<core[992335a751240df9]::result::Result<rustc_middle[e4acdad16398451c]::traits::select::EvaluationResult, rustc_middle[e4acdad16398451c]::traits::select::OverflowError>, <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::evaluate_root_obligation::{closure#0}>::{closure#0}>
  40:     0x7f126aecbbdc - <rustc_trait_selection[a9542b9bf4f2282b]::traits::select::SelectionContext>::evaluate_root_obligation
  41:     0x7f1269cf44f8 - <rustc_infer[28c90180cdc353a6]::infer::InferCtxtBuilder>::enter_with_canonical::<rustc_middle[e4acdad16398451c]::ty::ParamEnvAnd<rustc_middle[e4acdad16398451c]::ty::Predicate>, core[992335a751240df9]::result::Result<rustc_middle[e4acdad16398451c]::traits::select::EvaluationResult, rustc_middle[e4acdad16398451c]::traits::select::OverflowError>, rustc_traits[d50e600d694c30a]::evaluate_obligation::evaluate_obligation::{closure#0}>
  42:     0x7f1269cc721c - rustc_traits[d50e600d694c30a]::evaluate_obligation::evaluate_obligation
  43:     0x7f126a338fd2 - <rustc_query_system[bac8b8e6cbe04b6]::query::config::QueryVtable<rustc_query_impl[1637cc171863d90f]::plumbing::QueryCtxt, rustc_middle[e4acdad16398451c]::infer::canonical::Canonical<rustc_middle[e4acdad16398451c]::ty::ParamEnvAnd<rustc_middle[e4acdad16398451c]::ty::Predicate>>, core[992335a751240df9]::result::Result<rustc_middle[e4acdad16398451c]::traits::select::EvaluationResult, rustc_middle[e4acdad16398451c]::traits::select::OverflowError>>>::compute
  44:     0x7f126a533934 - rustc_query_system[bac8b8e6cbe04b6]::query::plumbing::get_query::<rustc_query_impl[1637cc171863d90f]::queries::evaluate_obligation, rustc_query_impl[1637cc171863d90f]::plumbing::QueryCtxt>
  45:     0x7f126a09327d - <rustc_query_impl[1637cc171863d90f]::Queries as rustc_middle[e4acdad16398451c]::ty::query::QueryEngine>::evaluate_obligation
  46:     0x7f126aeb6bd3 - <rustc_infer[28c90180cdc353a6]::infer::InferCtxt as rustc_trait_selection[a9542b9bf4f2282b]::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation
  47:     0x7f126aeb6e0d - <rustc_infer[28c90180cdc353a6]::infer::InferCtxt as rustc_trait_selection[a9542b9bf4f2282b]::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation_no_overflow
  48:     0x7f126aeb6947 - <rustc_infer[28c90180cdc353a6]::infer::InferCtxt as rustc_trait_selection[a9542b9bf4f2282b]::traits::query::evaluate_obligation::InferCtxtExt>::predicate_may_hold
  49:     0x7f12693df326 - <rustc_infer[28c90180cdc353a6]::infer::InferCtxt>::probe::<rustc_typeck[956e19107821422a]::check::method::probe::ProbeResult, <rustc_typeck[956e19107821422a]::check::method::probe::ProbeContext>::consider_probe::{closure#0}>
  50:     0x7f1269269332 - <rustc_typeck[956e19107821422a]::check::method::probe::ProbeContext>::consider_probe
  51:     0x7f1269442e06 - <core[992335a751240df9]::iter::adapters::map::Map<core[992335a751240df9]::slice::iter::Iter<rustc_typeck[956e19107821422a]::check::method::probe::Candidate>, <rustc_typeck[956e19107821422a]::check::method::probe::ProbeContext>::consider_candidates<core[992335a751240df9]::slice::iter::Iter<rustc_typeck[956e19107821422a]::check::method::probe::Candidate>>::{closure#0}> as core[992335a751240df9]::iter::traits::iterator::Iterator>::try_fold::<(), core[992335a751240df9]::iter::traits::iterator::Iterator::find::check<(&rustc_typeck[956e19107821422a]::check::method::probe::Candidate, rustc_typeck[956e19107821422a]::check::method::probe::ProbeResult), &mut <rustc_typeck[956e19107821422a]::check::method::probe::ProbeContext>::consider_candidates<core[992335a751240df9]::slice::iter::Iter<rustc_typeck[956e19107821422a]::check::method::probe::Candidate>>::{closure#1}>::{closure#0}, core[992335a751240df9]::ops::control_flow::ControlFlow<(&rustc_typeck[956e19107821422a]::check::method::probe::Candidate, rustc_typeck[956e19107821422a]::check::method::probe::ProbeResult)>>
  52:     0x7f1269493a88 - <alloc[29a2a132425083cc]::vec::Vec<(&rustc_typeck[956e19107821422a]::check::method::probe::Candidate, rustc_typeck[956e19107821422a]::check::method::probe::ProbeResult)> as alloc[29a2a132425083cc]::vec::spec_from_iter::SpecFromIter<(&rustc_typeck[956e19107821422a]::check::method::probe::Candidate, rustc_typeck[956e19107821422a]::check::method::probe::ProbeResult), core[992335a751240df9]::iter::adapters::filter::Filter<core[992335a751240df9]::iter::adapters::map::Map<core[992335a751240df9]::slice::iter::Iter<rustc_typeck[956e19107821422a]::check::method::probe::Candidate>, <rustc_typeck[956e19107821422a]::check::method::probe::ProbeContext>::consider_candidates<core[992335a751240df9]::slice::iter::Iter<rustc_typeck[956e19107821422a]::check::method::probe::Candidate>>::{closure#0}>, <rustc_typeck[956e19107821422a]::check::method::probe::ProbeContext>::consider_candidates<core[992335a751240df9]::slice::iter::Iter<rustc_typeck[956e19107821422a]::check::method::probe::Candidate>>::{closure#1}>>>::from_iter
  53:     0x7f1269268a55 - <rustc_typeck[956e19107821422a]::check::method::probe::ProbeContext>::consider_candidates::<core[992335a751240df9]::slice::iter::Iter<rustc_typeck[956e19107821422a]::check::method::probe::Candidate>>
  54:     0x7f1269267123 - <rustc_typeck[956e19107821422a]::check::method::probe::ProbeContext>::pick_method
  55:     0x7f12692660c9 - <rustc_typeck[956e19107821422a]::check::method::probe::ProbeContext>::pick_all_method
  56:     0x7f12692657c0 - <rustc_typeck[956e19107821422a]::check::method::probe::ProbeContext>::pick_core
  57:     0x7f12692649b4 - <rustc_typeck[956e19107821422a]::check::method::probe::ProbeContext>::pick
  58:     0x7f12693dbf86 - <rustc_infer[28c90180cdc353a6]::infer::InferCtxt>::probe::<core[992335a751240df9]::result::Result<rustc_typeck[956e19107821422a]::check::method::probe::Pick, rustc_typeck[956e19107821422a]::check::method::MethodError>, <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::probe_op<<rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::probe_for_return_type::{closure#1}::{closure#0}, rustc_typeck[956e19107821422a]::check::method::probe::Pick>::{closure#4}>
  59:     0x7f12691ce200 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::probe_op::<<rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::probe_for_return_type::{closure#1}::{closure#0}, rustc_typeck[956e19107821422a]::check::method::probe::Pick>
  60:     0x7f1269249cee - <&mut <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::probe_for_return_type::{closure#1} as core[992335a751240df9]::ops::function::FnOnce<(&rustc_span[b3e4704f910b2b73]::symbol::Ident,)>>::call_once
  61:     0x7f1269485726 - <alloc[29a2a132425083cc]::vec::Vec<rustc_middle[e4acdad16398451c]::ty::assoc::AssocItem> as alloc[29a2a132425083cc]::vec::spec_from_iter::SpecFromIter<rustc_middle[e4acdad16398451c]::ty::assoc::AssocItem, core[992335a751240df9]::iter::adapters::flatten::FlatMap<core[992335a751240df9]::slice::iter::Iter<rustc_span[b3e4704f910b2b73]::symbol::Ident>, core[992335a751240df9]::option::Option<rustc_middle[e4acdad16398451c]::ty::assoc::AssocItem>, <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::probe_for_return_type::{closure#1}>>>::from_iter
  62:     0x7f126921e1b6 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::probe_for_return_type
  63:     0x7f12691c571e - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::suggest_deref_ref_or_into
  64:     0x7f12691967b7 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::emit_coerce_suggestions
  65:     0x7f126920baad - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::demand_coerce_diag
  66:     0x7f12692124b2 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_expr_kind
  67:     0x7f12691a3ee0 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  68:     0x7f126920be49 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  69:     0x7f12691c2afe - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_stmt
  70:     0x7f12691c3084 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  71:     0x7f126920d341 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_expr_kind
  72:     0x7f12691a3ee0 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  73:     0x7f126920be49 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  74:     0x7f12691a57c9 - <rustc_typeck[956e19107821422a]::check::fn_ctxt::FnCtxt>::check_return_expr
  75:     0x7f1269413399 - rustc_typeck[956e19107821422a]::check::check::check_fn
  76:     0x7f12693c730a - <rustc_infer[28c90180cdc353a6]::infer::InferCtxtBuilder>::enter::<&rustc_middle[e4acdad16398451c]::ty::context::TypeckResults, <rustc_typeck[956e19107821422a]::check::inherited::InheritedBuilder>::enter<rustc_typeck[956e19107821422a]::check::typeck_with_fallback<rustc_typeck[956e19107821422a]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[e4acdad16398451c]::ty::context::TypeckResults>::{closure#0}>
  77:     0x7f12692e459e - rustc_typeck[956e19107821422a]::check::typeck
  78:     0x7f126a44c83d - rustc_query_system[bac8b8e6cbe04b6]::query::plumbing::try_execute_query::<rustc_query_impl[1637cc171863d90f]::plumbing::QueryCtxt, rustc_query_system[bac8b8e6cbe04b6]::query::caches::DefaultCache<rustc_span[b3e4704f910b2b73]::def_id::LocalDefId, &rustc_middle[e4acdad16398451c]::ty::context::TypeckResults>>
  79:     0x7f126a56cfd7 - rustc_query_system[bac8b8e6cbe04b6]::query::plumbing::get_query::<rustc_query_impl[1637cc171863d90f]::queries::typeck, rustc_query_impl[1637cc171863d90f]::plumbing::QueryCtxt>
  80:     0x7f126a05edf4 - <rustc_query_impl[1637cc171863d90f]::Queries as rustc_middle[e4acdad16398451c]::ty::query::QueryEngine>::typeck
  81:     0x7f1269282065 - rustc_typeck[956e19107821422a]::collect::type_of::opt_const_param_of
  82:     0x7f126a43dfa5 - rustc_query_system[bac8b8e6cbe04b6]::query::plumbing::try_execute_query::<rustc_query_impl[1637cc171863d90f]::plumbing::QueryCtxt, rustc_query_system[bac8b8e6cbe04b6]::query::caches::DefaultCache<rustc_span[b3e4704f910b2b73]::def_id::LocalDefId, core[992335a751240df9]::option::Option<rustc_span[b3e4704f910b2b73]::def_id::DefId>>>
  83:     0x7f126a532a0e - rustc_query_system[bac8b8e6cbe04b6]::query::plumbing::get_query::<rustc_query_impl[1637cc171863d90f]::queries::opt_const_param_of, rustc_query_impl[1637cc171863d90f]::plumbing::QueryCtxt>
  84:     0x7f126a041474 - <rustc_query_impl[1637cc171863d90f]::Queries as rustc_middle[e4acdad16398451c]::ty::query::QueryEngine>::opt_const_param_of
  85:     0x7f12692e3ed1 - rustc_typeck[956e19107821422a]::check::typeck
  86:     0x7f126a44c83d - rustc_query_system[bac8b8e6cbe04b6]::query::plumbing::try_execute_query::<rustc_query_impl[1637cc171863d90f]::plumbing::QueryCtxt, rustc_query_system[bac8b8e6cbe04b6]::query::caches::DefaultCache<rustc_span[b3e4704f910b2b73]::def_id::LocalDefId, &rustc_middle[e4acdad16398451c]::ty::context::TypeckResults>>
  87:     0x7f126a56cfd7 - rustc_query_system[bac8b8e6cbe04b6]::query::plumbing::get_query::<rustc_query_impl[1637cc171863d90f]::queries::typeck, rustc_query_impl[1637cc171863d90f]::plumbing::QueryCtxt>
  88:     0x7f126a05edf4 - <rustc_query_impl[1637cc171863d90f]::Queries as rustc_middle[e4acdad16398451c]::ty::query::QueryEngine>::typeck
  89:     0x7f12694a6fca - <rustc_middle[e4acdad16398451c]::hir::map::Map>::par_body_owners::<rustc_typeck[956e19107821422a]::check::typeck_item_bodies::{closure#0}>
  90:     0x7f12692e99bd - rustc_typeck[956e19107821422a]::check::typeck_item_bodies
  91:     0x7f126a492cbf - rustc_query_system[bac8b8e6cbe04b6]::query::plumbing::try_execute_query::<rustc_query_impl[1637cc171863d90f]::plumbing::QueryCtxt, rustc_query_system[bac8b8e6cbe04b6]::query::caches::DefaultCache<(), ()>>
  92:     0x7f126a532b35 - rustc_query_system[bac8b8e6cbe04b6]::query::plumbing::get_query::<rustc_query_impl[1637cc171863d90f]::queries::typeck_item_bodies, rustc_query_impl[1637cc171863d90f]::plumbing::QueryCtxt>
  93:     0x7f126a05e89e - <rustc_query_impl[1637cc171863d90f]::Queries as rustc_middle[e4acdad16398451c]::ty::query::QueryEngine>::typeck_item_bodies
  94:     0x7f12693054ca - <rustc_session[c3b8890bf519571]::session::Session>::time::<(), rustc_typeck[956e19107821422a]::check_crate::{closure#7}>
  95:     0x7f12694da5ae - rustc_typeck[956e19107821422a]::check_crate
  96:     0x7f12689a33a1 - rustc_interface[71f375b04d6f2c83]::passes::analysis
  97:     0x7f126a487a40 - rustc_query_system[bac8b8e6cbe04b6]::query::plumbing::try_execute_query::<rustc_query_impl[1637cc171863d90f]::plumbing::QueryCtxt, rustc_query_system[bac8b8e6cbe04b6]::query::caches::DefaultCache<(), core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>>>
  98:     0x7f126a56d3b2 - rustc_query_system[bac8b8e6cbe04b6]::query::plumbing::get_query::<rustc_query_impl[1637cc171863d90f]::queries::analysis, rustc_query_impl[1637cc171863d90f]::plumbing::QueryCtxt>
  99:     0x7f126a0424ae - <rustc_query_impl[1637cc171863d90f]::Queries as rustc_middle[e4acdad16398451c]::ty::query::QueryEngine>::analysis
 100:     0x7f12688e3925 - <rustc_interface[71f375b04d6f2c83]::passes::QueryContext>::enter::<rustc_driver[f852af0d696e410b]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>>
 101:     0x7f1268894e5c - <rustc_interface[71f375b04d6f2c83]::interface::Compiler>::enter::<rustc_driver[f852af0d696e410b]::run_compiler::{closure#1}::{closure#2}, core[992335a751240df9]::result::Result<core[992335a751240df9]::option::Option<rustc_interface[71f375b04d6f2c83]::queries::Linker>, rustc_errors[782abd4b611703e6]::ErrorGuaranteed>>
 102:     0x7f126887ce48 - rustc_span[b3e4704f910b2b73]::with_source_map::<core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>, rustc_interface[71f375b04d6f2c83]::interface::create_compiler_and_run<core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>, rustc_driver[f852af0d696e410b]::run_compiler::{closure#1}>::{closure#1}>
 103:     0x7f1268896d29 - rustc_interface[71f375b04d6f2c83]::interface::create_compiler_and_run::<core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>, rustc_driver[f852af0d696e410b]::run_compiler::{closure#1}>
 104:     0x7f1268876e5f - <scoped_tls[e65d73f394969a0b]::ScopedKey<rustc_span[b3e4704f910b2b73]::SessionGlobals>>::set::<rustc_interface[71f375b04d6f2c83]::interface::run_compiler<core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>, rustc_driver[f852af0d696e410b]::run_compiler::{closure#1}>::{closure#0}, core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>>
 105:     0x7f1268880ba9 - std[c99ebb468f9d244]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[71f375b04d6f2c83]::util::run_in_thread_pool_with_globals<rustc_interface[71f375b04d6f2c83]::interface::run_compiler<core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>, rustc_driver[f852af0d696e410b]::run_compiler::{closure#1}>::{closure#0}, core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>>::{closure#0}, core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>>
 106:     0x7f12688eac79 - <<std[c99ebb468f9d244]::thread::Builder>::spawn_unchecked_<rustc_interface[71f375b04d6f2c83]::util::run_in_thread_pool_with_globals<rustc_interface[71f375b04d6f2c83]::interface::run_compiler<core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>, rustc_driver[f852af0d696e410b]::run_compiler::{closure#1}>::{closure#0}, core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>>::{closure#0}, core[992335a751240df9]::result::Result<(), rustc_errors[782abd4b611703e6]::ErrorGuaranteed>>::{closure#1} as core[992335a751240df9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
 107:     0x7f1267edf303 - std::sys::unix::thread::Thread::new::thread_start::h66a50617e17509ef
 108:     0x7f126242b609 - start_thread
 109:     0x7f1267d3e133 - clone
 110:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (b61ef4fde 2022-07-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `[u8; _]: alloc::borrow::ToOwned`
#1 [typeck] type-checking `catch_me`
#2 [opt_const_param_of] computing the optional const parameter of `catch_me::{constant#1}`
#3 [typeck] type-checking `catch_me::{constant#1}`
#4 [typeck_item_bodies] type-checking all item bodies
#5 [analysis] running analysis passes on this crate
error: aborting due to previous error
------------------------------------------


