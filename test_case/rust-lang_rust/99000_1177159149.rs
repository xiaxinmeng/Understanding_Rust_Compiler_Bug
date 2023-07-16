plain
.........................i.....ii....................................................... 1760/13154
........................................................................................ 1848/13154
...............................................................................i........ 1936/13154
........................................................................................ 2024/13154
.......................................................F............F.............F..... 2112/13154
...F..F................................................................................. 2200/13154
.....................................................................................F.. 2288/13154
......................................F................................................. 2376/13154
........................................................................................ 2552/13154
........................................................................................ 2640/13154
........................................................................................ 2728/13154
........................................................................................ 2816/13154
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
stack backtrace:
   0:     0x7f130bea5bac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb1f58dc9c546a135
   1:     0x7f130bf0a768 - core::fmt::write::h2566d6e89e6599a1
   2:     0x7f130be96211 - std::io::Write::write_fmt::h82125ed3b9775fed
   3:     0x7f130bea8a5e - std::panicking::default_hook::{{closure}}::h1879ffb239823c1b
   4:     0x7f130bea8746 - std::panicking::default_hook::h90cf0db515c71230
   5:     0x7f130c8590e4 - rustc_driver[8ee5c2a5d44e4852]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f130bea91f2 - std::panicking::rust_panic_with_hook::hce68862497d8aeb3
   7:     0x7f130f1525b3 - std[3af2761e2449ed4e]::panicking::begin_panic::<rustc_errors[45cf9f74fdf60c75]::ExplicitBug>::{closure#0}
   8:     0x7f130f150516 - std[3af2761e2449ed4e]::sys_common::backtrace::__rust_end_short_backtrace::<std[3af2761e2449ed4e]::panicking::begin_panic<rustc_errors[45cf9f74fdf60c75]::ExplicitBug>::{closure#0}, !>
   9:     0x7f130c7ee566 - std[3af2761e2449ed4e]::panicking::begin_panic::<rustc_errors[45cf9f74fdf60c75]::ExplicitBug>
  10:     0x7f130f2ba686 - std[3af2761e2449ed4e]::panic::panic_any::<rustc_errors[45cf9f74fdf60c75]::ExplicitBug>
  11:     0x7f130f2b8c96 - <rustc_errors[45cf9f74fdf60c75]::HandlerInner>::bug::<&alloc[7d3da87ed62233d4]::string::String>
  12:     0x7f130f2b8950 - <rustc_errors[45cf9f74fdf60c75]::Handler>::bug::<&alloc[7d3da87ed62233d4]::string::String>
  13:     0x7f130f32cc85 - rustc_middle[25b31cceba598e71]::util::bug::opt_span_bug_fmt::<rustc_span[bf154f15c703a8f0]::span_encoding::Span>::{closure#0}
  14:     0x7f130f32c43b - rustc_middle[25b31cceba598e71]::ty::context::tls::with_opt::<rustc_middle[25b31cceba598e71]::util::bug::opt_span_bug_fmt<rustc_span[bf154f15c703a8f0]::span_encoding::Span>::{closure#0}, ()>::{closure#0}
  15:     0x7f130f32c3e6 - rustc_middle[25b31cceba598e71]::ty::context::tls::with_context_opt::<rustc_middle[25b31cceba598e71]::ty::context::tls::with_opt<rustc_middle[25b31cceba598e71]::util::bug::opt_span_bug_fmt<rustc_span[bf154f15c703a8f0]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  16:     0x7f130f32cbc9 - rustc_middle[25b31cceba598e71]::util::bug::opt_span_bug_fmt::<rustc_span[bf154f15c703a8f0]::span_encoding::Span>
  17:     0x7f130c7f5b75 - rustc_middle[25b31cceba598e71]::util::bug::bug_fmt
  18:     0x7f130f2a9cfb - <rustc_middle[25b31cceba598e71]::ty::context::TyCtxt>::const_eval_resolve_for_typeck
  19:     0x7f130f022b98 - <rustc_infer[4a910b1980ad3e5f]::infer::InferCtxt>::const_eval_resolve
  20:     0x7f130f009509 - <rustc_infer[4a910b1980ad3e5f]::infer::InferCtxt>::try_const_eval_resolve
  21:     0x7f130ee63b68 - <rustc_trait_selection[53841b84636c2402]::traits::select::SelectionContext>::evaluate_predicate_recursively
  22:     0x7f130ee626f9 - <rustc_trait_selection[53841b84636c2402]::traits::select::SelectionContext>::evaluate_predicates_recursively::<alloc[7d3da87ed62233d4]::vec::into_iter::IntoIter<rustc_infer[4a910b1980ad3e5f]::traits::Obligation<rustc_middle[25b31cceba598e71]::ty::Predicate>>>
  23:     0x7f130edfddff - <rustc_infer[4a910b1980ad3e5f]::infer::InferCtxt>::probe::<core[acc016d383ccc355]::result::Result<rustc_middle[25b31cceba598e71]::traits::select::EvaluationResult, rustc_middle[25b31cceba598e71]::traits::select::OverflowError>, <rustc_trait_selection[53841b84636c2402]::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection[53841b84636c2402]::traits::select::SelectionContext>::evaluate_candidate::{closure#0}>::{closure#0}>
  24:     0x7f130ee68634 - <rustc_trait_selection[53841b84636c2402]::traits::select::SelectionContext>::evaluate_candidate
  25:     0x7f130eef7cb8 - <core[acc016d383ccc355]::iter::adapters::map::Map<core[acc016d383ccc355]::iter::adapters::map::Map<alloc[7d3da87ed62233d4]::vec::into_iter::IntoIter<rustc_middle[25b31cceba598e71]::traits::select::SelectionCandidate>, <rustc_trait_selection[53841b84636c2402]::traits::select::SelectionContext>::candidate_from_obligation_no_cache::{closure#0}>, <core[acc016d383ccc355]::result::Result<core[acc016d383ccc355]::option::Option<rustc_trait_selection[53841b84636c2402]::traits::select::EvaluatedCandidate>, rustc_middle[25b31cceba598e71]::traits::SelectionError>>::transpose> as core[acc016d383ccc355]::iter::traits::iterator::Iterator>::try_fold::<(), <core[acc016d383ccc355]::iter::adapters::flatten::FlattenCompat<_, _> as core[acc016d383ccc355]::iter::traits::iterator::Iterator>::try_fold::flatten<core[acc016d383ccc355]::option::Option<core[acc016d383ccc355]::result::Result<rustc_trait_selection[53841b84636c2402]::traits::select::EvaluatedCandidate, rustc_middle[25b31cceba598e71]::traits::SelectionError>>, (), core[acc016d383ccc355]::ops::control_flow::ControlFlow<core[acc016d383ccc355]::ops::control_flow::ControlFlow<rustc_trait_selection[53841b84636c2402]::traits::select::EvaluatedCandidate>>, <core[acc016d383ccc355]::iter::adapters::GenericShunt<core[acc016d383ccc355]::iter::adapters::flatten::FlatMap<core[acc016d383ccc355]::iter::adapters::map::Map<alloc[7d3da87ed62233d4]::vec::into_iter::IntoIter<rustc_middle[25b31cceba598e71]::traits::select::SelectionCandidate>, <rustc_trait_selection[53841b84636c2402]::traits::select::SelectionContext>::candidate_from_obligation_no_cache::{closure#0}>, core[acc016d383ccc355]::option::Option<core[acc016d383ccc355]::result::Result<rustc_trait_selection[53841b84636c2402]::traits::select::EvaluatedCandidate, rustc_middle[25b31cceba598e71]::traits::SelectionError>>, <core[acc016d383ccc355]::result::Result<core[acc016d383ccc355]::option::Option<rustc_trait_selection[53841b84636c2402]::traits::select::EvaluatedCandidate>, rustc_middle[25b31cceba598e71]::traits::SelectionError>>::transpose>, core[acc016d383ccc355]::result::Result<core[acc016d383ccc355]::convert::Infallible, rustc_middle[25b31cceba598e71]::traits::SelectionError>> as core[acc016d383ccc355]::iter::traits::iterator::Iterator>::try_fold<(), core[acc016d383ccc355]::iter::traits::iterator::Iterator::try_for_each::call<rustc_trait_selection[53841b84636c2402]::traits::select::EvaluatedCandidate, core[acc016d383ccc355]::ops::control_flow::ControlFlow<rustc_trait_selection[53841b84636c2402]::traits::select::EvaluatedCandidate>, core[acc016d383ccc355]::ops::control_flow::ControlFlow<rustc_trait_selection[53841b84636c2402]::traits::select::EvaluatedCandidate>::Break>::{closure#0}, core[acc016d383ccc355]::ops::control_flow::ControlFlow<rustc_trait_selection[53841b84636c2402]::traits::select::EvaluatedCandidate>>::{closure#0}>::{closure#0}, core[acc016d383ccc355]::ops::control_flow::ControlFlow<core[acc016d383ccc355]::ops::control_flow::ControlFlow<rustc_trait_selection[53841b84636c2402]::traits::select::EvaluatedCandidate>>>
  26:     0x7f130efa3889 - <core[acc016d383ccc355]::iter::adapters::GenericShunt<core[acc016d383ccc355]::iter::adapters::flatten::FlatMap<core[acc016d383ccc355]::iter::adapters::map::Map<alloc[7d3da87ed62233d4]::vec::into_iter::IntoIter<rustc_middle[25b31cceba598e71]::traits::select::SelectionCandidate>, <rustc_trait_selection[53841b84636c2402]::traits::select::SelectionContext>::candidate_from_obligation_no_cache::{closure#0}>, core[acc016d383ccc355]::option::Option<core[acc016d383ccc355]::result::Result<rustc_trait_selection[53841b84636c2402]::traits::select::EvaluatedCandidate, rustc_middle[25b31cceba598e71]::traits::SelectionError>>, <core[acc016d383ccc355]::result::Result<core[acc016d383ccc355]::option::Option<rustc_trait_selection[53841b84636c2402]::traits::select::EvaluatedCandidate>, rustc_middle[25b31cceba598e71]::traits::SelectionError>>::transpose>, core[acc016d383ccc355]::result::Result<core[acc016d383ccc355]::convert::Infallible, rustc_middle[25b31cceba598e71]::traits::SelectionError>> as core[acc016d383ccc355]::iter::traits::iterator::Iterator>::next
  27:     0x7f130ee82f27 - <alloc[7d3da87ed62233d4]::vec::Vec<rustc_trait_selection[53841b84636c2402]::traits::select::EvaluatedCandidate> as alloc[7d3da87ed62233d4]::vec::spec_from_iter::SpecFromIter<rustc_trait_selection[53841b84636c2402]::traits::select::EvaluatedCandidate, core[acc016d383ccc355]::iter::adapters::GenericShunt<core[acc016d383ccc355]::iter::adapters::flatten::FlatMap<core[acc016d383ccc355]::iter::adapters::map::Map<alloc[7d3da87ed62233d4]::vec::into_iter::IntoIter<rustc_middle[25b31cceba598e71]::traits::select::SelectionCandidate>, <rustc_trait_selection[53841b84636c2402]::traits::select::SelectionContext>::candidate_from_obligation_no_cache::{closure#0}>, core[acc016d383ccc355]::option::Option<core[acc016d383ccc355]::result::Result<rustc_trait_selection[53841b84636c2402]::traits::select::EvaluatedCandidate, rustc_middle[25b31cceba598e71]::traits::SelectionError>>, <core[acc016d383ccc355]::result::Result<core[acc016d383ccc355]::option::Option<rustc_trait_selection[53841b84636c2402]::traits::select::EvaluatedCandidate>, rustc_middle[25b31cceba598e71]::traits::SelectionError>>::transpose>, core[acc016d383ccc355]::result::Result<core[acc016d383ccc355]::convert::Infallible, rustc_middle[25b31cceba598e71]::traits::SelectionError>>>>::from_iter
  28:     0x7f130ef9b93d - core[acc016d383ccc355]::iter::adapters::try_process::<core[acc016d383ccc355]::iter::adapters::flatten::FlatMap<core[acc016d383ccc355]::iter::adapters::map::Map<alloc[7d3da87ed62233d4]::vec::into_iter::IntoIter<rustc_middle[25b31cceba598e71]::traits::select::SelectionCandidate>, <rustc_trait_selection[53841b84636c2402]::traits::select::SelectionContext>::candidate_from_obligation_no_cache::{closure#0}>, core[acc016d383ccc355]::option::Option<core[acc016d383ccc355]::result::Result<rustc_trait_selection[53841b84636c2402]::traits::select::EvaluatedCandidate, rustc_middle[25b31cceba598e71]::traits::SelectionError>>, <core[acc016d383ccc355]::result::Result<core[acc016d383ccc355]::option::Option<rustc_trait_selection[53841b84636c2402]::traits::select::EvaluatedCandidate>, rustc_middle[25b31cceba598e71]::traits::SelectionError>>::transpose>, rustc_trait_selection[53841b84636c2402]::traits::select::EvaluatedCandidate, core[acc016d383ccc355]::result::Result<core[acc016d383ccc355]::convert::Infallible, rustc_middle[25b31cceba598e71]::traits::SelectionError>, <core[acc016d383ccc355]::result::Result<alloc[7d3da87ed62233d4]::vec::Vec<rustc_trait_selection[53841b84636c2402]::traits::select::EvaluatedCandidate>, rustc_middle[25b31cceba598e71]::traits::SelectionError> as core[acc016d383ccc355]::iter::traits::collect::FromIterator<core[acc016d383ccc355]::result::Result<rustc_trait_selection[53841b84636c2402]::traits::select::EvaluatedCandidate, rustc_middle[25b31cceba598e71]::traits::SelectionError>>>::from_iter<core[acc016d383ccc355]::iter::adapters::flatten::FlatMap<core[acc016d383ccc355]::iter::adapters::map::Map<alloc[7d3da87ed62233d4]::vec::into_iter::IntoIter<rustc_middle[25b31cceba598e71]::traits::select::SelectionCandidate>, <rustc_trait_selection[53841b84636c2402]::traits::select::SelectionContext>::candidate_from_obligation_no_cache::{closure#0}>, core[acc016d383ccc355]::option::Option<core[acc016d383ccc355]::result::Result<rustc_trait_selection[53841b84636c2402]::traits::select::EvaluatedCandidate, rustc_middle[25b31cceba598e71]::traits::SelectionError>>, <core[acc016d383ccc355]::result::Result<core[acc016d383ccc355]::option::Option<rustc_trait_selection[53841b84636c2402]::traits::select::EvaluatedCandidate>, rustc_middle[25b31cceba598e71]::traits::SelectionError>>::transpose>>::{closure#0}, alloc[7d3da87ed62233d4]::vec::Vec<rustc_trait_selection[53841b84636c2402]::traits::select::EvaluatedCandidate>>
  29:     0x7f130ee423b4 - <rustc_trait_selection[53841b84636c2402]::traits::select::SelectionContext>::candidate_from_obligation_no_cache
  30:     0x7f130eec21ad - <rustc_query_system[17f9a051e15e89d6]::dep_graph::graph::DepGraph<rustc_middle[25b31cceba598e71]::dep_graph::dep_node::DepKind>>::with_anon_task::<rustc_middle[25b31cceba598e71]::ty::context::TyCtxt, <rustc_trait_selection[53841b84636c2402]::traits::select::SelectionContext>::in_task<<rustc_trait_selection[53841b84636c2402]::traits::select::SelectionContext>::candidate_from_obligation::{closure#0}, core[acc016d383ccc355]::result::Result<core[acc016d383ccc355]::option::Option<rustc_middle[25b31cceba598e71]::traits::select::SelectionCandidate>, rustc_middle[25b31cceba598e71]::traits::SelectionError>>::{closure#0}, core[acc016d383ccc355]::result::Result<core[acc016d383ccc355]::option::Option<rustc_middle[25b31cceba598e71]::traits::select::SelectionCandidate>, rustc_middle[25b31cceba598e71]::traits::SelectionError>>
  31:     0x7f130ee4c1e2 - <rustc_trait_selection[53841b84636c2402]::traits::select::SelectionContext>::candidate_from_obligation
  32:     0x7f130ee47216 - <rustc_trait_selection[53841b84636c2402]::traits::select::SelectionContext>::select_from_obligation
  33:     0x7f130ee61427 - <rustc_trait_selection[53841b84636c2402]::traits::select::SelectionContext>::select
  34:     0x7f130ef1aee0 - <rustc_trait_selection[53841b84636c2402]::traits::fulfill::FulfillProcessor>::process_trait_obligation
  35:     0x7f130ef096c9 - <rustc_trait_selection[53841b84636c2402]::traits::fulfill::FulfillProcessor as rustc_data_structures[28cd5455da4b8e58]::obligation_forest::ObligationProcessor>::process_obligation
  36:     0x7f130ef98bab - <rustc_data_structures[28cd5455da4b8e58]::obligation_forest::ObligationForest<rustc_trait_selection[53841b84636c2402]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[53841b84636c2402]::traits::fulfill::FulfillProcessor, rustc_data_structures[28cd5455da4b8e58]::obligation_forest::Outcome<rustc_trait_selection[53841b84636c2402]::traits::fulfill::PendingPredicateObligation, rustc_infer[4a910b1980ad3e5f]::traits::FulfillmentErrorCode>>
  37:     0x7f130ef087ca - <rustc_trait_selection[53841b84636c2402]::traits::fulfill::FulfillmentContext as rustc_infer[4a910b1980ad3e5f]::traits::engine::TraitEngine>::select_where_possible
  38:     0x7f130d16dab1 - <rustc_typeck[63ecf0077c45e275]::check::fn_ctxt::FnCtxt>::resolve_vars_with_obligations
  39:     0x7f130d172e1a - <rustc_typeck[63ecf0077c45e275]::check::fn_ctxt::FnCtxt>::structurally_resolved_type
  40:     0x7f130d1459e0 - <rustc_typeck[63ecf0077c45e275]::check::fn_ctxt::FnCtxt>::check_call
  41:     0x7f130d1c65aa - <rustc_typeck[63ecf0077c45e275]::check::fn_ctxt::FnCtxt>::check_expr_kind
  42:     0x7f130d15d7d4 - <rustc_typeck[63ecf0077c45e275]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  43:     0x7f130d1c5349 - <rustc_typeck[63ecf0077c45e275]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  44:     0x7f130d1cb66c - <rustc_typeck[63ecf0077c45e275]::check::fn_ctxt::FnCtxt>::check_expr_kind
  45:     0x7f130d15d7d4 - <rustc_typeck[63ecf0077c45e275]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  46:     0x7f130d1c5349 - <rustc_typeck[63ecf0077c45e275]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  47:     0x7f130d17c06e - <rustc_typeck[63ecf0077c45e275]::check::fn_ctxt::FnCtxt>::check_stmt
  48:     0x7f130d17c5f4 - <rustc_typeck[63ecf0077c45e275]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  49:     0x7f130d1c6840 - <rustc_typeck[63ecf0077c45e275]::check::fn_ctxt::FnCtxt>::check_expr_kind
  50:     0x7f130d15d7d4 - <rustc_typeck[63ecf0077c45e275]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  51:     0x7f130d1c5349 - <rustc_typeck[63ecf0077c45e275]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  52:     0x7f130d15ec69 - <rustc_typeck[63ecf0077c45e275]::check::fn_ctxt::FnCtxt>::check_return_expr
  53:     0x7f130d3bd880 - rustc_typeck[63ecf0077c45e275]::check::check::check_fn
  54:     0x7f130d1f6fbb - <rustc_infer[4a910b1980ad3e5f]::infer::InferCtxtBuilder>::enter::<&rustc_middle[25b31cceba598e71]::ty::context::TypeckResults, <rustc_typeck[63ecf0077c45e275]::check::inherited::InheritedBuilder>::enter<rustc_typeck[63ecf0077c45e275]::check::typeck_with_fallback<rustc_typeck[63ecf0077c45e275]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[25b31cceba598e71]::ty::context::TypeckResults>::{closure#0}>
  55:     0x7f130d2b4fae - <rustc_typeck[63ecf0077c45e275]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[63ecf0077c45e275]::check::typeck_with_fallback<rustc_typeck[63ecf0077c45e275]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[25b31cceba598e71]::ty::context::TypeckResults>
  56:     0x7f130d28afa3 - rustc_typeck[63ecf0077c45e275]::check::typeck
  57:     0x7f130e3d34dc - rustc_query_system[17f9a051e15e89d6]::query::plumbing::try_execute_query::<rustc_query_impl[1d37261add045b1f]::plumbing::QueryCtxt, rustc_query_system[17f9a051e15e89d6]::query::caches::DefaultCache<rustc_span[bf154f15c703a8f0]::def_id::LocalDefId, &rustc_middle[25b31cceba598e71]::ty::context::TypeckResults>>
  58:     0x7f130e4ee907 - rustc_query_system[17f9a051e15e89d6]::query::plumbing::get_query::<rustc_query_impl[1d37261add045b1f]::queries::typeck, rustc_query_impl[1d37261add045b1f]::plumbing::QueryCtxt>
  59:     0x7f130dfe53e4 - <rustc_query_impl[1d37261add045b1f]::Queries as rustc_middle[25b31cceba598e71]::ty::query::QueryEngine>::typeck
  60:     0x7f130d255f4c - rustc_typeck[63ecf0077c45e275]::collect::type_of::opt_const_param_of
  61:     0x7f130e3c52f5 - rustc_query_system[17f9a051e15e89d6]::query::plumbing::try_execute_query::<rustc_query_impl[1d37261add045b1f]::plumbing::QueryCtxt, rustc_query_system[17f9a051e15e89d6]::query::caches::DefaultCache<rustc_span[bf154f15c703a8f0]::def_id::LocalDefId, core[acc016d383ccc355]::option::Option<rustc_span[bf154f15c703a8f0]::def_id::DefId>>>
  62:     0x7f130e4b320e - rustc_query_system[17f9a051e15e89d6]::query::plumbing::get_query::<rustc_query_impl[1d37261add045b1f]::queries::opt_const_param_of, rustc_query_impl[1d37261add045b1f]::plumbing::QueryCtxt>
  63:     0x7f130dfc7a64 - <rustc_query_impl[1d37261add045b1f]::Queries as rustc_middle[25b31cceba598e71]::ty::query::QueryEngine>::opt_const_param_of
  64:     0x7f130d28a901 - rustc_typeck[63ecf0077c45e275]::check::typeck
  65:     0x7f130e3d34dc - rustc_query_system[17f9a051e15e89d6]::query::plumbing::try_execute_query::<rustc_query_impl[1d37261add045b1f]::plumbing::QueryCtxt, rustc_query_system[17f9a051e15e89d6]::query::caches::DefaultCache<rustc_span[bf154f15c703a8f0]::def_id::LocalDefId, &rustc_middle[25b31cceba598e71]::ty::context::TypeckResults>>
  66:     0x7f130e4ee907 - rustc_query_system[17f9a051e15e89d6]::query::plumbing::get_query::<rustc_query_impl[1d37261add045b1f]::queries::typeck, rustc_query_impl[1d37261add045b1f]::plumbing::QueryCtxt>
  67:     0x7f130dfe53e4 - <rustc_query_impl[1d37261add045b1f]::Queries as rustc_middle[25b31cceba598e71]::ty::query::QueryEngine>::typeck
  68:     0x7f130d44ed5a - <rustc_middle[25b31cceba598e71]::hir::map::Map>::par_body_owners::<rustc_typeck[63ecf0077c45e275]::check::typeck_item_bodies::{closure#0}>
  69:     0x7f130d2906ed - rustc_typeck[63ecf0077c45e275]::check::typeck_item_bodies
  70:     0x7f130e41c0c1 - rustc_query_system[17f9a051e15e89d6]::query::plumbing::try_execute_query::<rustc_query_impl[1d37261add045b1f]::plumbing::QueryCtxt, rustc_query_system[17f9a051e15e89d6]::query::caches::DefaultCache<(), ()>>
  71:     0x7f130e4b3335 - rustc_query_system[17f9a051e15e89d6]::query::plumbing::get_query::<rustc_query_impl[1d37261add045b1f]::queries::typeck_item_bodies, rustc_query_impl[1d37261add045b1f]::plumbing::QueryCtxt>
  72:     0x7f130dfe4e8e - <rustc_query_impl[1d37261add045b1f]::Queries as rustc_middle[25b31cceba598e71]::ty::query::QueryEngine>::typeck_item_bodies
  73:     0x7f130d2fa0aa - <rustc_session[eed827af599b72c5]::session::Session>::time::<(), rustc_typeck[63ecf0077c45e275]::check_crate::{closure#7}>
  74:     0x7f130d47feee - rustc_typeck[63ecf0077c45e275]::check_crate
  75:     0x7f130c96c781 - rustc_interface[330ab41afc3a5249]::passes::analysis
  76:     0x7f130e410734 - rustc_query_system[17f9a051e15e89d6]::query::plumbing::try_execute_query::<rustc_query_impl[1d37261add045b1f]::plumbing::QueryCtxt, rustc_query_system[17f9a051e15e89d6]::query::caches::DefaultCache<(), core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>>
  77:     0x7f130e4eece2 - rustc_query_system[17f9a051e15e89d6]::query::plumbing::get_query::<rustc_query_impl[1d37261add045b1f]::queries::analysis, rustc_query_impl[1d37261add045b1f]::plumbing::QueryCtxt>
  78:     0x7f130dfc8a9e - <rustc_query_impl[1d37261add045b1f]::Queries as rustc_middle[25b31cceba598e71]::ty::query::QueryEngine>::analysis
  79:     0x7f130c8ad905 - <rustc_interface[330ab41afc3a5249]::passes::QueryContext>::enter::<rustc_driver[8ee5c2a5d44e4852]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>
  80:     0x7f130c8608bc - <rustc_interface[330ab41afc3a5249]::interface::Compiler>::enter::<rustc_driver[8ee5c2a5d44e4852]::run_compiler::{closure#1}::{closure#2}, core[acc016d383ccc355]::result::Result<core[acc016d383ccc355]::option::Option<rustc_interface[330ab41afc3a5249]::queries::Linker>, rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>
  81:     0x7f130c8481d8 - rustc_span[bf154f15c703a8f0]::with_source_map::<core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>, rustc_interface[330ab41afc3a5249]::interface::create_compiler_and_run<core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>, rustc_driver[8ee5c2a5d44e4852]::run_compiler::{closure#1}>::{closure#1}>
  82:     0x7f130c862499 - rustc_interface[330ab41afc3a5249]::interface::create_compiler_and_run::<core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>, rustc_driver[8ee5c2a5d44e4852]::run_compiler::{closure#1}>
  83:     0x7f130c84224f - <scoped_tls[4824bcaa7ff46d8a]::ScopedKey<rustc_span[bf154f15c703a8f0]::SessionGlobals>>::set::<rustc_interface[330ab41afc3a5249]::interface::run_compiler<core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>, rustc_driver[8ee5c2a5d44e4852]::run_compiler::{closure#1}>::{closure#0}, core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>
  84:     0x7f130c84c2f9 - std[3af2761e2449ed4e]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[330ab41afc3a5249]::util::run_in_thread_pool_with_globals<rustc_interface[330ab41afc3a5249]::interface::run_compiler<core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>, rustc_driver[8ee5c2a5d44e4852]::run_compiler::{closure#1}>::{closure#0}, core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>::{closure#0}, core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>
  85:     0x7f130c8b54a9 - <<std[3af2761e2449ed4e]::thread::Builder>::spawn_unchecked_<rustc_interface[330ab41afc3a5249]::util::run_in_thread_pool_with_globals<rustc_interface[330ab41afc3a5249]::interface::run_compiler<core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>, rustc_driver[8ee5c2a5d44e4852]::run_compiler::{closure#1}>::{closure#0}, core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>::{closure#0}, core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>::{closure#1} as core[acc016d383ccc355]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  86:     0x7f130beb40e3 - std::sys::unix::thread::Thread::new::thread_start::h2e419d554d1bcff7
  87:     0x7f1306402609 - start_thread
  88:     0x7f130bd15133 - clone
  89:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (b11794869 2022-07-06) running on x86_64-unknown-linux-gnu

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
stack backtrace:
   0:     0x7f30f94cabac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb1f58dc9c546a135
   1:     0x7f30f952f768 - core::fmt::write::h2566d6e89e6599a1
   2:     0x7f30f94bb211 - std::io::Write::write_fmt::h82125ed3b9775fed
   3:     0x7f30f94cda5e - std::panicking::default_hook::{{closure}}::h1879ffb239823c1b
   4:     0x7f30f94cd746 - std::panicking::default_hook::h90cf0db515c71230
   5:     0x7f30f9e7e0e4 - rustc_driver[8ee5c2a5d44e4852]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f30f94ce1f2 - std::panicking::rust_panic_with_hook::hce68862497d8aeb3
   7:     0x7f30fc7775b3 - std[3af2761e2449ed4e]::panicking::begin_panic::<rustc_errors[45cf9f74fdf60c75]::ExplicitBug>::{closure#0}
   8:     0x7f30fc775516 - std[3af2761e2449ed4e]::sys_common::backtrace::__rust_end_short_backtrace::<std[3af2761e2449ed4e]::panicking::begin_panic<rustc_errors[45cf9f74fdf60c75]::ExplicitBug>::{closure#0}, !>
   9:     0x7f30f9e13566 - std[3af2761e2449ed4e]::panicking::begin_panic::<rustc_errors[45cf9f74fdf60c75]::ExplicitBug>
  10:     0x7f30fc8df686 - std[3af2761e2449ed4e]::panic::panic_any::<rustc_errors[45cf9f74fdf60c75]::ExplicitBug>
  11:     0x7f30fc8ddc96 - <rustc_errors[45cf9f74fdf60c75]::HandlerInner>::bug::<&alloc[7d3da87ed62233d4]::string::String>
  12:     0x7f30fc8dd950 - <rustc_errors[45cf9f74fdf60c75]::Handler>::bug::<&alloc[7d3da87ed62233d4]::string::String>
  13:     0x7f30fc951c85 - rustc_middle[25b31cceba598e71]::util::bug::opt_span_bug_fmt::<rustc_span[bf154f15c703a8f0]::span_encoding::Span>::{closure#0}
  14:     0x7f30fc95143b - rustc_middle[25b31cceba598e71]::ty::context::tls::with_opt::<rustc_middle[25b31cceba598e71]::util::bug::opt_span_bug_fmt<rustc_span[bf154f15c703a8f0]::span_encoding::Span>::{closure#0}, ()>::{closure#0}
  15:     0x7f30fc9513e6 - rustc_middle[25b31cceba598e71]::ty::context::tls::with_context_opt::<rustc_middle[25b31cceba598e71]::ty::context::tls::with_opt<rustc_middle[25b31cceba598e71]::util::bug::opt_span_bug_fmt<rustc_span[bf154f15c703a8f0]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  16:     0x7f30fc951bc9 - rustc_middle[25b31cceba598e71]::util::bug::opt_span_bug_fmt::<rustc_span[bf154f15c703a8f0]::span_encoding::Span>
  17:     0x7f30f9e1ab75 - rustc_middle[25b31cceba598e71]::util::bug::bug_fmt
  18:     0x7f30fc8cecfb - <rustc_middle[25b31cceba598e71]::ty::context::TyCtxt>::const_eval_resolve_for_typeck
  19:     0x7f30fc647b98 - <rustc_infer[4a910b1980ad3e5f]::infer::InferCtxt>::const_eval_resolve
  20:     0x7f30fc648b04 - rustc_infer[4a910b1980ad3e5f]::infer::is_const_evaluatable
  21:     0x7f30fc52e824 - <rustc_trait_selection[53841b84636c2402]::traits::fulfill::FulfillProcessor as rustc_data_structures[28cd5455da4b8e58]::obligation_forest::ObligationProcessor>::process_obligation
  22:     0x7f30fc5bdbab - <rustc_data_structures[28cd5455da4b8e58]::obligation_forest::ObligationForest<rustc_trait_selection[53841b84636c2402]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[53841b84636c2402]::traits::fulfill::FulfillProcessor, rustc_data_structures[28cd5455da4b8e58]::obligation_forest::Outcome<rustc_trait_selection[53841b84636c2402]::traits::fulfill::PendingPredicateObligation, rustc_infer[4a910b1980ad3e5f]::traits::FulfillmentErrorCode>>
  23:     0x7f30fc52d7ca - <rustc_trait_selection[53841b84636c2402]::traits::fulfill::FulfillmentContext as rustc_infer[4a910b1980ad3e5f]::traits::engine::TraitEngine>::select_where_possible
  24:     0x7f30fa799568 - <rustc_typeck[63ecf0077c45e275]::check::fn_ctxt::FnCtxt>::check_argument_types
  25:     0x7f30fa76d608 - <rustc_typeck[63ecf0077c45e275]::check::fn_ctxt::FnCtxt>::confirm_builtin_call
  26:     0x7f30fa76b4bb - <rustc_typeck[63ecf0077c45e275]::check::fn_ctxt::FnCtxt>::check_call
  27:     0x7f30fa7eb5aa - <rustc_typeck[63ecf0077c45e275]::check::fn_ctxt::FnCtxt>::check_expr_kind
  28:     0x7f30fa7827d4 - <rustc_typeck[63ecf0077c45e275]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  29:     0x7f30fa7ea349 - <rustc_typeck[63ecf0077c45e275]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  30:     0x7f30fa7a0cf5 - <rustc_typeck[63ecf0077c45e275]::check::fn_ctxt::FnCtxt>::check_decl
  31:     0x7f30fa7a0efa - <rustc_typeck[63ecf0077c45e275]::check::fn_ctxt::FnCtxt>::check_stmt
  32:     0x7f30fa7a15f4 - <rustc_typeck[63ecf0077c45e275]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  33:     0x7f30fa7eb840 - <rustc_typeck[63ecf0077c45e275]::check::fn_ctxt::FnCtxt>::check_expr_kind
  34:     0x7f30fa7827d4 - <rustc_typeck[63ecf0077c45e275]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  35:     0x7f30fa7ea349 - <rustc_typeck[63ecf0077c45e275]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  36:     0x7f30fa783c69 - <rustc_typeck[63ecf0077c45e275]::check::fn_ctxt::FnCtxt>::check_return_expr
  37:     0x7f30fa9e2880 - rustc_typeck[63ecf0077c45e275]::check::check::check_fn
  38:     0x7f30fa81bfbb - <rustc_infer[4a910b1980ad3e5f]::infer::InferCtxtBuilder>::enter::<&rustc_middle[25b31cceba598e71]::ty::context::TypeckResults, <rustc_typeck[63ecf0077c45e275]::check::inherited::InheritedBuilder>::enter<rustc_typeck[63ecf0077c45e275]::check::typeck_with_fallback<rustc_typeck[63ecf0077c45e275]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[25b31cceba598e71]::ty::context::TypeckResults>::{closure#0}>
  39:     0x7f30fa8d9fae - <rustc_typeck[63ecf0077c45e275]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[63ecf0077c45e275]::check::typeck_with_fallback<rustc_typeck[63ecf0077c45e275]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[25b31cceba598e71]::ty::context::TypeckResults>
  40:     0x7f30fa8affa3 - rustc_typeck[63ecf0077c45e275]::check::typeck
  41:     0x7f30fb9f84dc - rustc_query_system[17f9a051e15e89d6]::query::plumbing::try_execute_query::<rustc_query_impl[1d37261add045b1f]::plumbing::QueryCtxt, rustc_query_system[17f9a051e15e89d6]::query::caches::DefaultCache<rustc_span[bf154f15c703a8f0]::def_id::LocalDefId, &rustc_middle[25b31cceba598e71]::ty::context::TypeckResults>>
  42:     0x7f30fbb13907 - rustc_query_system[17f9a051e15e89d6]::query::plumbing::get_query::<rustc_query_impl[1d37261add045b1f]::queries::typeck, rustc_query_impl[1d37261add045b1f]::plumbing::QueryCtxt>
  43:     0x7f30fb60a3e4 - <rustc_query_impl[1d37261add045b1f]::Queries as rustc_middle[25b31cceba598e71]::ty::query::QueryEngine>::typeck
  44:     0x7f30faa73d5a - <rustc_middle[25b31cceba598e71]::hir::map::Map>::par_body_owners::<rustc_typeck[63ecf0077c45e275]::check::typeck_item_bodies::{closure#0}>
  45:     0x7f30fa8b56ed - rustc_typeck[63ecf0077c45e275]::check::typeck_item_bodies
  46:     0x7f30fba410c1 - rustc_query_system[17f9a051e15e89d6]::query::plumbing::try_execute_query::<rustc_query_impl[1d37261add045b1f]::plumbing::QueryCtxt, rustc_query_system[17f9a051e15e89d6]::query::caches::DefaultCache<(), ()>>
  47:     0x7f30fbad8335 - rustc_query_system[17f9a051e15e89d6]::query::plumbing::get_query::<rustc_query_impl[1d37261add045b1f]::queries::typeck_item_bodies, rustc_query_impl[1d37261add045b1f]::plumbing::QueryCtxt>
  48:     0x7f30fb609e8e - <rustc_query_impl[1d37261add045b1f]::Queries as rustc_middle[25b31cceba598e71]::ty::query::QueryEngine>::typeck_item_bodies
  49:     0x7f30fa91f0aa - <rustc_session[eed827af599b72c5]::session::Session>::time::<(), rustc_typeck[63ecf0077c45e275]::check_crate::{closure#7}>
  50:     0x7f30faaa4eee - rustc_typeck[63ecf0077c45e275]::check_crate
  51:     0x7f30f9f91781 - rustc_interface[330ab41afc3a5249]::passes::analysis
  52:     0x7f30fba35734 - rustc_query_system[17f9a051e15e89d6]::query::plumbing::try_execute_query::<rustc_query_impl[1d37261add045b1f]::plumbing::QueryCtxt, rustc_query_system[17f9a051e15e89d6]::query::caches::DefaultCache<(), core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>>
  53:     0x7f30fbb13ce2 - rustc_query_system[17f9a051e15e89d6]::query::plumbing::get_query::<rustc_query_impl[1d37261add045b1f]::queries::analysis, rustc_query_impl[1d37261add045b1f]::plumbing::QueryCtxt>
  54:     0x7f30fb5eda9e - <rustc_query_impl[1d37261add045b1f]::Queries as rustc_middle[25b31cceba598e71]::ty::query::QueryEngine>::analysis
  55:     0x7f30f9ed2905 - <rustc_interface[330ab41afc3a5249]::passes::QueryContext>::enter::<rustc_driver[8ee5c2a5d44e4852]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>
  56:     0x7f30f9e858bc - <rustc_interface[330ab41afc3a5249]::interface::Compiler>::enter::<rustc_driver[8ee5c2a5d44e4852]::run_compiler::{closure#1}::{closure#2}, core[acc016d383ccc355]::result::Result<core[acc016d383ccc355]::option::Option<rustc_interface[330ab41afc3a5249]::queries::Linker>, rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>
  57:     0x7f30f9e6d1d8 - rustc_span[bf154f15c703a8f0]::with_source_map::<core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>, rustc_interface[330ab41afc3a5249]::interface::create_compiler_and_run<core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>, rustc_driver[8ee5c2a5d44e4852]::run_compiler::{closure#1}>::{closure#1}>
  58:     0x7f30f9e87499 - rustc_interface[330ab41afc3a5249]::interface::create_compiler_and_run::<core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>, rustc_driver[8ee5c2a5d44e4852]::run_compiler::{closure#1}>
  59:     0x7f30f9e6724f - <scoped_tls[4824bcaa7ff46d8a]::ScopedKey<rustc_span[bf154f15c703a8f0]::SessionGlobals>>::set::<rustc_interface[330ab41afc3a5249]::interface::run_compiler<core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>, rustc_driver[8ee5c2a5d44e4852]::run_compiler::{closure#1}>::{closure#0}, core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>
  60:     0x7f30f9e712f9 - std[3af2761e2449ed4e]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[330ab41afc3a5249]::util::run_in_thread_pool_with_globals<rustc_interface[330ab41afc3a5249]::interface::run_compiler<core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>, rustc_driver[8ee5c2a5d44e4852]::run_compiler::{closure#1}>::{closure#0}, core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>::{closure#0}, core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>
  61:     0x7f30f9eda4a9 - <<std[3af2761e2449ed4e]::thread::Builder>::spawn_unchecked_<rustc_interface[330ab41afc3a5249]::util::run_in_thread_pool_with_globals<rustc_interface[330ab41afc3a5249]::interface::run_compiler<core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>, rustc_driver[8ee5c2a5d44e4852]::run_compiler::{closure#1}>::{closure#0}, core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>::{closure#0}, core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>::{closure#1} as core[acc016d383ccc355]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  62:     0x7f30f94d90e3 - std::sys::unix::thread::Thread::new::thread_start::h2e419d554d1bcff7
  63:     0x7f30f3a27609 - start_thread
  64:     0x7f30f933a133 - clone
  65:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (b11794869 2022-07-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/issue-76595.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-76595.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-76595" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-76595/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0107]: this function takes 2 generic arguments but 1 generic argument was supplied
   |
LL |     test::<2>();
LL |     test::<2>();
   |     ^^^^   - supplied 1 generic argument
   |     expected 2 generic arguments
   |
   |
note: function defined here, with 2 generic parameters: `T`, `P`
   |
   |
LL | fn test<T, const P: usize>() where Bool<{core::mem::size_of::<T>() > 4}>: True {
   |    ^^^^ -  --------------
help: add missing generic argument
   |
LL |     test::<2, P>();


error: internal compiler error: compiler/rustc_middle/src/mir/interpret/queries.rs:76:13: did not expect inference variables here
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1391:9
stack backtrace:
stack backtrace:
   0:     0x7f93395ebbac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb1f58dc9c546a135
   1:     0x7f9339650768 - core::fmt::write::h2566d6e89e6599a1
   2:     0x7f93395dc211 - std::io::Write::write_fmt::h82125ed3b9775fed
   3:     0x7f93395eea5e - std::panicking::default_hook::{{closure}}::h1879ffb239823c1b
   4:     0x7f93395ee746 - std::panicking::default_hook::h90cf0db515c71230
   5:     0x7f9339f9f0e4 - rustc_driver[8ee5c2a5d44e4852]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f93395ef1f2 - std::panicking::rust_panic_with_hook::hce68862497d8aeb3
   7:     0x7f933c8985b3 - std[3af2761e2449ed4e]::panicking::begin_panic::<rustc_errors[45cf9f74fdf60c75]::ExplicitBug>::{closure#0}
   8:     0x7f933c896516 - std[3af2761e2449ed4e]::sys_common::backtrace::__rust_end_short_backtrace::<std[3af2761e2449ed4e]::panicking::begin_panic<rustc_errors[45cf9f74fdf60c75]::ExplicitBug>::{closure#0}, !>
   9:     0x7f9339f34566 - std[3af2761e2449ed4e]::panicking::begin_panic::<rustc_errors[45cf9f74fdf60c75]::ExplicitBug>
  10:     0x7f933ca00686 - std[3af2761e2449ed4e]::panic::panic_any::<rustc_errors[45cf9f74fdf60c75]::ExplicitBug>
  11:     0x7f933c9fec96 - <rustc_errors[45cf9f74fdf60c75]::HandlerInner>::bug::<&alloc[7d3da87ed62233d4]::string::String>
  12:     0x7f933c9fe950 - <rustc_errors[45cf9f74fdf60c75]::Handler>::bug::<&alloc[7d3da87ed62233d4]::string::String>
  13:     0x7f933ca72c85 - rustc_middle[25b31cceba598e71]::util::bug::opt_span_bug_fmt::<rustc_span[bf154f15c703a8f0]::span_encoding::Span>::{closure#0}
  14:     0x7f933ca7243b - rustc_middle[25b31cceba598e71]::ty::context::tls::with_opt::<rustc_middle[25b31cceba598e71]::util::bug::opt_span_bug_fmt<rustc_span[bf154f15c703a8f0]::span_encoding::Span>::{closure#0}, ()>::{closure#0}
  15:     0x7f933ca723e6 - rustc_middle[25b31cceba598e71]::ty::context::tls::with_context_opt::<rustc_middle[25b31cceba598e71]::ty::context::tls::with_opt<rustc_middle[25b31cceba598e71]::util::bug::opt_span_bug_fmt<rustc_span[bf154f15c703a8f0]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  16:     0x7f933ca72bc9 - rustc_middle[25b31cceba598e71]::util::bug::opt_span_bug_fmt::<rustc_span[bf154f15c703a8f0]::span_encoding::Span>
  17:     0x7f9339f3bb75 - rustc_middle[25b31cceba598e71]::util::bug::bug_fmt
  18:     0x7f933c9efcfb - <rustc_middle[25b31cceba598e71]::ty::context::TyCtxt>::const_eval_resolve_for_typeck
  19:     0x7f933c768b98 - <rustc_infer[4a910b1980ad3e5f]::infer::InferCtxt>::const_eval_resolve
  20:     0x7f933c769b04 - rustc_infer[4a910b1980ad3e5f]::infer::is_const_evaluatable
  21:     0x7f933c64f824 - <rustc_trait_selection[53841b84636c2402]::traits::fulfill::FulfillProcessor as rustc_data_structures[28cd5455da4b8e58]::obligation_forest::ObligationProcessor>::process_obligation
  22:     0x7f933c6debab - <rustc_data_structures[28cd5455da4b8e58]::obligation_forest::ObligationForest<rustc_trait_selection[53841b84636c2402]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[53841b84636c2402]::traits::fulfill::FulfillProcessor, rustc_data_structures[28cd5455da4b8e58]::obligation_forest::Outcome<rustc_trait_selection[53841b84636c2402]::traits::fulfill::PendingPredicateObligation, rustc_infer[4a910b1980ad3e5f]::traits::FulfillmentErrorCode>>
  23:     0x7f933c64e7ca - <rustc_trait_selection[53841b84636c2402]::traits::fulfill::FulfillmentContext as rustc_infer[4a910b1980ad3e5f]::traits::engine::TraitEngine>::select_where_possible
  24:     0x7f933a8b0f98 - <rustc_typeck[63ecf0077c45e275]::check::fn_ctxt::FnCtxt>::type_inference_fallback
  25:     0x7f933a93d203 - <rustc_infer[4a910b1980ad3e5f]::infer::InferCtxtBuilder>::enter::<&rustc_middle[25b31cceba598e71]::ty::context::TypeckResults, <rustc_typeck[63ecf0077c45e275]::check::inherited::InheritedBuilder>::enter<rustc_typeck[63ecf0077c45e275]::check::typeck_with_fallback<rustc_typeck[63ecf0077c45e275]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[25b31cceba598e71]::ty::context::TypeckResults>::{closure#0}>
  26:     0x7f933a9fafae - <rustc_typeck[63ecf0077c45e275]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[63ecf0077c45e275]::check::typeck_with_fallback<rustc_typeck[63ecf0077c45e275]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[25b31cceba598e71]::ty::context::TypeckResults>
  27:     0x7f933a9d0fa3 - rustc_typeck[63ecf0077c45e275]::check::typeck
  28:     0x7f933bb194dc - rustc_query_system[17f9a051e15e89d6]::query::plumbing::try_execute_query::<rustc_query_impl[1d37261add045b1f]::plumbing::QueryCtxt, rustc_query_system[17f9a051e15e89d6]::query::caches::DefaultCache<rustc_span[bf154f15c703a8f0]::def_id::LocalDefId, &rustc_middle[25b31cceba598e71]::ty::context::TypeckResults>>
  29:     0x7f933bc34907 - rustc_query_system[17f9a051e15e89d6]::query::plumbing::get_query::<rustc_query_impl[1d37261add045b1f]::queries::typeck, rustc_query_impl[1d37261add045b1f]::plumbing::QueryCtxt>
  30:     0x7f933b72b3e4 - <rustc_query_impl[1d37261add045b1f]::Queries as rustc_middle[25b31cceba598e71]::ty::query::QueryEngine>::typeck
  31:     0x7f933a99bf4c - rustc_typeck[63ecf0077c45e275]::collect::type_of::opt_const_param_of
  32:     0x7f933bb0b2f5 - rustc_query_system[17f9a051e15e89d6]::query::plumbing::try_execute_query::<rustc_query_impl[1d37261add045b1f]::plumbing::QueryCtxt, rustc_query_system[17f9a051e15e89d6]::query::caches::DefaultCache<rustc_span[bf154f15c703a8f0]::def_id::LocalDefId, core[acc016d383ccc355]::option::Option<rustc_span[bf154f15c703a8f0]::def_id::DefId>>>
  33:     0x7f933bbf920e - rustc_query_system[17f9a051e15e89d6]::query::plumbing::get_query::<rustc_query_impl[1d37261add045b1f]::queries::opt_const_param_of, rustc_query_impl[1d37261add045b1f]::plumbing::QueryCtxt>
  34:     0x7f933b70da64 - <rustc_query_impl[1d37261add045b1f]::Queries as rustc_middle[25b31cceba598e71]::ty::query::QueryEngine>::opt_const_param_of
  35:     0x7f933a9d0901 - rustc_typeck[63ecf0077c45e275]::check::typeck
  36:     0x7f933bb194dc - rustc_query_system[17f9a051e15e89d6]::query::plumbing::try_execute_query::<rustc_query_impl[1d37261add045b1f]::plumbing::QueryCtxt, rustc_query_system[17f9a051e15e89d6]::query::caches::DefaultCache<rustc_span[bf154f15c703a8f0]::def_id::LocalDefId, &rustc_middle[25b31cceba598e71]::ty::context::TypeckResults>>
  37:     0x7f933bc34907 - rustc_query_system[17f9a051e15e89d6]::query::plumbing::get_query::<rustc_query_impl[1d37261add045b1f]::queries::typeck, rustc_query_impl[1d37261add045b1f]::plumbing::QueryCtxt>
  38:     0x7f933b72b3e4 - <rustc_query_impl[1d37261add045b1f]::Queries as rustc_middle[25b31cceba598e71]::ty::query::QueryEngine>::typeck
  39:     0x7f933ab94d5a - <rustc_middle[25b31cceba598e71]::hir::map::Map>::par_body_owners::<rustc_typeck[63ecf0077c45e275]::check::typeck_item_bodies::{closure#0}>
  40:     0x7f933a9d66ed - rustc_typeck[63ecf0077c45e275]::check::typeck_item_bodies
  41:     0x7f933bb620c1 - rustc_query_system[17f9a051e15e89d6]::query::plumbing::try_execute_query::<rustc_query_impl[1d37261add045b1f]::plumbing::QueryCtxt, rustc_query_system[17f9a051e15e89d6]::query::caches::DefaultCache<(), ()>>
  42:     0x7f933bbf9335 - rustc_query_system[17f9a051e15e89d6]::query::plumbing::get_query::<rustc_query_impl[1d37261add045b1f]::queries::typeck_item_bodies, rustc_query_impl[1d37261add045b1f]::plumbing::QueryCtxt>
  43:     0x7f933b72ae8e - <rustc_query_impl[1d37261add045b1f]::Queries as rustc_middle[25b31cceba598e71]::ty::query::QueryEngine>::typeck_item_bodies
  44:     0x7f933aa400aa - <rustc_session[eed827af599b72c5]::session::Session>::time::<(), rustc_typeck[63ecf0077c45e275]::check_crate::{closure#7}>
  45:     0x7f933abc5eee - rustc_typeck[63ecf0077c45e275]::check_crate
  46:     0x7f933a0b2781 - rustc_interface[330ab41afc3a5249]::passes::analysis
  47:     0x7f933bb56734 - rustc_query_system[17f9a051e15e89d6]::query::plumbing::try_execute_query::<rustc_query_impl[1d37261add045b1f]::plumbing::QueryCtxt, rustc_query_system[17f9a051e15e89d6]::query::caches::DefaultCache<(), core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>>
  48:     0x7f933bc34ce2 - rustc_query_system[17f9a051e15e89d6]::query::plumbing::get_query::<rustc_query_impl[1d37261add045b1f]::queries::analysis, rustc_query_impl[1d37261add045b1f]::plumbing::QueryCtxt>
  49:     0x7f933b70ea9e - <rustc_query_impl[1d37261add045b1f]::Queries as rustc_middle[25b31cceba598e71]::ty::query::QueryEngine>::analysis
  50:     0x7f9339ff3905 - <rustc_interface[330ab41afc3a5249]::passes::QueryContext>::enter::<rustc_driver[8ee5c2a5d44e4852]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>
  51:     0x7f9339fa68bc - <rustc_interface[330ab41afc3a5249]::interface::Compiler>::enter::<rustc_driver[8ee5c2a5d44e4852]::run_compiler::{closure#1}::{closure#2}, core[acc016d383ccc355]::result::Result<core[acc016d383ccc355]::option::Option<rustc_interface[330ab41afc3a5249]::queries::Linker>, rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>
  52:     0x7f9339f8e1d8 - rustc_span[bf154f15c703a8f0]::with_source_map::<core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>, rustc_interface[330ab41afc3a5249]::interface::create_compiler_and_run<core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>, rustc_driver[8ee5c2a5d44e4852]::run_compiler::{closure#1}>::{closure#1}>
  53:     0x7f9339fa8499 - rustc_interface[330ab41afc3a5249]::interface::create_compiler_and_run::<core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>, rustc_driver[8ee5c2a5d44e4852]::run_compiler::{closure#1}>
  54:     0x7f9339f8824f - <scoped_tls[4824bcaa7ff46d8a]::ScopedKey<rustc_span[bf154f15c703a8f0]::SessionGlobals>>::set::<rustc_interface[330ab41afc3a5249]::interface::run_compiler<core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>, rustc_driver[8ee5c2a5d44e4852]::run_compiler::{closure#1}>::{closure#0}, core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>
  55:     0x7f9339f922f9 - std[3af2761e2449ed4e]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[330ab41afc3a5249]::util::run_in_thread_pool_with_globals<rustc_interface[330ab41afc3a5249]::interface::run_compiler<core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>, rustc_driver[8ee5c2a5d44e4852]::run_compiler::{closure#1}>::{closure#0}, core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>::{closure#0}, core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>
  56:     0x7f9339ffb4a9 - <<std[3af2761e2449ed4e]::thread::Builder>::spawn_unchecked_<rustc_interface[330ab41afc3a5249]::util::run_in_thread_pool_with_globals<rustc_interface[330ab41afc3a5249]::interface::run_compiler<core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>, rustc_driver[8ee5c2a5d44e4852]::run_compiler::{closure#1}>::{closure#0}, core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>::{closure#0}, core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>::{closure#1} as core[acc016d383ccc355]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  57:     0x7f93395fa0e3 - std::sys::unix::thread::Thread::new::thread_start::h2e419d554d1bcff7
  58:     0x7f9333b48609 - start_thread
  59:     0x7f933945b133 - clone
  60:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (b11794869 2022-07-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [opt_const_param_of] computing the optional const parameter of `main::{constant#0}`
#2 [typeck] type-checking `main::{constant#0}`
#3 [typeck_item_bodies] type-checking all item bodies
#4 [analysis] running analysis passes on this crate
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0107`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/const-generics/generic_const_exprs/issue-97047-ice-1.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-97047-ice-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-97047-ice-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-97047-ice-1/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `adt_const_params` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(adt_const_params, generic_const_exprs)]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #95174 <https://github.com/rust-lang/rust/issues/95174> for more information


warning: the feature `generic_const_exprs` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(adt_const_params, generic_const_exprs)]
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information

warning: 2 warnings emitted
warning: 2 warnings emitted

error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in DefId(0:11 ~ issue_97047_ice_1[da0f]::{impl#0}::new) (NoSolution): could not prove Binder(WellFormed([(); _]), [])
   |
LL | /         Self {
LL | /         Self {
LL | |             changes: [0; CHANGES.len()],
LL | |         }
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:151:13


error: internal compiler error: broken MIR in DefId(0:11 ~ issue_97047_ice_1[da0f]::{impl#0}::new) (NoSolution): could not prove Binder(ConstEvaluatable(WithOptConstParam { did: DefId(0:5 ~ issue_97047_ice_1[da0f]::Changes::{constant#0}), const_param_did: None }, [Const { ty: &[&str], kind: Param(CHANGES/#0) }]), [])
   |
LL | /         Self {
LL | /         Self {
LL | |             changes: [0; CHANGES.len()],
LL | |         }
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:151:13

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
stack backtrace:
   0:     0x7f8e0bd33bac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb1f58dc9c546a135
   1:     0x7f8e0bd98768 - core::fmt::write::h2566d6e89e6599a1
   2:     0x7f8e0bd24211 - std::io::Write::write_fmt::h82125ed3b9775fed
   3:     0x7f8e0bd36a5e - std::panicking::default_hook::{{closure}}::h1879ffb239823c1b
   4:     0x7f8e0bd36746 - std::panicking::default_hook::h90cf0db515c71230
   5:     0x7f8e0c6e70e4 - rustc_driver[8ee5c2a5d44e4852]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f8e0bd371f2 - std::panicking::rust_panic_with_hook::hce68862497d8aeb3
   7:     0x7f8e0f3af323 - std[3af2761e2449ed4e]::panicking::begin_panic::<rustc_errors[45cf9f74fdf60c75]::ExplicitBug>::{closure#0}
   8:     0x7f8e0f3ae996 - std[3af2761e2449ed4e]::sys_common::backtrace::__rust_end_short_backtrace::<std[3af2761e2449ed4e]::panicking::begin_panic<rustc_errors[45cf9f74fdf60c75]::ExplicitBug>::{closure#0}, !>
   9:     0x7f8e0c6a3266 - std[3af2761e2449ed4e]::panicking::begin_panic::<rustc_errors[45cf9f74fdf60c75]::ExplicitBug>
  10:     0x7f8e0f3a32b6 - std[3af2761e2449ed4e]::panic::panic_any::<rustc_errors[45cf9f74fdf60c75]::ExplicitBug>
  11:     0x7f8e0f3a84bd - <rustc_errors[45cf9f74fdf60c75]::HandlerInner as core[acc016d383ccc355]::ops::drop::Drop>::drop
  12:     0x7f8e0c705472 - core[acc016d383ccc355]::ptr::drop_in_place::<rustc_session[eed827af599b72c5]::parse::ParseSess>
  13:     0x7f8e0c70d1ad - <alloc[7d3da87ed62233d4]::rc::Rc<rustc_session[eed827af599b72c5]::session::Session> as core[acc016d383ccc355]::ops::drop::Drop>::drop
  14:     0x7f8e0c6d93cc - core[acc016d383ccc355]::ptr::drop_in_place::<rustc_interface[330ab41afc3a5249]::interface::Compiler>
  15:     0x7f8e0c6d6399 - rustc_span[bf154f15c703a8f0]::with_source_map::<core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>, rustc_interface[330ab41afc3a5249]::interface::create_compiler_and_run<core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>, rustc_driver[8ee5c2a5d44e4852]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f8e0c6f0499 - rustc_interface[330ab41afc3a5249]::interface::create_compiler_and_run::<core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>, rustc_driver[8ee5c2a5d44e4852]::run_compiler::{closure#1}>
  17:     0x7f8e0c6d024f - <scoped_tls[4824bcaa7ff46d8a]::ScopedKey<rustc_span[bf154f15c703a8f0]::SessionGlobals>>::set::<rustc_interface[330ab41afc3a5249]::interface::run_compiler<core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>, rustc_driver[8ee5c2a5d44e4852]::run_compiler::{closure#1}>::{closure#0}, core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>
  18:     0x7f8e0c6da2f9 - std[3af2761e2449ed4e]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[330ab41afc3a5249]::util::run_in_thread_pool_with_globals<rustc_interface[330ab41afc3a5249]::interface::run_compiler<core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>, rustc_driver[8ee5c2a5d44e4852]::run_compiler::{closure#1}>::{closure#0}, core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>::{closure#0}, core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>
  19:     0x7f8e0c7434a9 - <<std[3af2761e2449ed4e]::thread::Builder>::spawn_unchecked_<rustc_interface[330ab41afc3a5249]::util::run_in_thread_pool_with_globals<rustc_interface[330ab41afc3a5249]::interface::run_compiler<core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>, rustc_driver[8ee5c2a5d44e4852]::run_compiler::{closure#1}>::{closure#0}, core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>::{closure#0}, core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>::{closure#1} as core[acc016d383ccc355]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  20:     0x7f8e0bd420e3 - std::sys::unix::thread::Thread::new::thread_start::h2e419d554d1bcff7
  21:     0x7f8e06290609 - start_thread
  22:     0x7f8e0bba3133 - clone
  23:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (b11794869 2022-07-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/issue-97047-ice-2.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-97047-ice-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-97047-ice-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-97047-ice-2/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `adt_const_params` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(adt_const_params, generic_const_exprs)]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #95174 <https://github.com/rust-lang/rust/issues/95174> for more information


warning: the feature `generic_const_exprs` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(adt_const_params, generic_const_exprs)]
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information

warning: 2 warnings emitted
warning: 2 warnings emitted

error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in DefId(0:11 ~ issue_97047_ice_2[19cb]::{impl#0}::combine) (NoSolution): could not prove Binder(WellFormed(&[usize; _]), [])
   |
   |
LL |         for _change in &self.changes {}
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:151:13

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
stack backtrace:
   0:     0x7f6f9fb5fbac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb1f58dc9c546a135
   1:     0x7f6f9fbc4768 - core::fmt::write::h2566d6e89e6599a1
   2:     0x7f6f9fb50211 - std::io::Write::write_fmt::h82125ed3b9775fed
   3:     0x7f6f9fb62a5e - std::panicking::default_hook::{{closure}}::h1879ffb239823c1b
   4:     0x7f6f9fb62746 - std::panicking::default_hook::h90cf0db515c71230
   5:     0x7f6fa05130e4 - rustc_driver[8ee5c2a5d44e4852]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f6f9fb631f2 - std::panicking::rust_panic_with_hook::hce68862497d8aeb3
   7:     0x7f6fa31db323 - std[3af2761e2449ed4e]::panicking::begin_panic::<rustc_errors[45cf9f74fdf60c75]::ExplicitBug>::{closure#0}
   8:     0x7f6fa31da996 - std[3af2761e2449ed4e]::sys_common::backtrace::__rust_end_short_backtrace::<std[3af2761e2449ed4e]::panicking::begin_panic<rustc_errors[45cf9f74fdf60c75]::ExplicitBug>::{closure#0}, !>
   9:     0x7f6fa04cf266 - std[3af2761e2449ed4e]::panicking::begin_panic::<rustc_errors[45cf9f74fdf60c75]::ExplicitBug>
  10:     0x7f6fa31cf2b6 - std[3af2761e2449ed4e]::panic::panic_any::<rustc_errors[45cf9f74fdf60c75]::ExplicitBug>
  11:     0x7f6fa31d44bd - <rustc_errors[45cf9f74fdf60c75]::HandlerInner as core[acc016d383ccc355]::ops::drop::Drop>::drop
  12:     0x7f6fa0531472 - core[acc016d383ccc355]::ptr::drop_in_place::<rustc_session[eed827af599b72c5]::parse::ParseSess>
  13:     0x7f6fa05391ad - <alloc[7d3da87ed62233d4]::rc::Rc<rustc_session[eed827af599b72c5]::session::Session> as core[acc016d383ccc355]::ops::drop::Drop>::drop
  14:     0x7f6fa05053cc - core[acc016d383ccc355]::ptr::drop_in_place::<rustc_interface[330ab41afc3a5249]::interface::Compiler>
  15:     0x7f6fa0502399 - rustc_span[bf154f15c703a8f0]::with_source_map::<core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>, rustc_interface[330ab41afc3a5249]::interface::create_compiler_and_run<core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>, rustc_driver[8ee5c2a5d44e4852]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f6fa051c499 - rustc_interface[330ab41afc3a5249]::interface::create_compiler_and_run::<core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>, rustc_driver[8ee5c2a5d44e4852]::run_compiler::{closure#1}>
  17:     0x7f6fa04fc24f - <scoped_tls[4824bcaa7ff46d8a]::ScopedKey<rustc_span[bf154f15c703a8f0]::SessionGlobals>>::set::<rustc_interface[330ab41afc3a5249]::interface::run_compiler<core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>, rustc_driver[8ee5c2a5d44e4852]::run_compiler::{closure#1}>::{closure#0}, core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>
  18:     0x7f6fa05062f9 - std[3af2761e2449ed4e]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[330ab41afc3a5249]::util::run_in_thread_pool_with_globals<rustc_interface[330ab41afc3a5249]::interface::run_compiler<core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>, rustc_driver[8ee5c2a5d44e4852]::run_compiler::{closure#1}>::{closure#0}, core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>::{closure#0}, core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>
  19:     0x7f6fa056f4a9 - <<std[3af2761e2449ed4e]::thread::Builder>::spawn_unchecked_<rustc_interface[330ab41afc3a5249]::util::run_in_thread_pool_with_globals<rustc_interface[330ab41afc3a5249]::interface::run_compiler<core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>, rustc_driver[8ee5c2a5d44e4852]::run_compiler::{closure#1}>::{closure#0}, core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>::{closure#0}, core[acc016d383ccc355]::result::Result<(), rustc_errors[45cf9f74fdf60c75]::ErrorGuaranteed>>::{closure#1} as core[acc016d383ccc355]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  20:     0x7f6f9fb6e0e3 - std::sys::unix::thread::Thread::new::thread_start::h2e419d554d1bcff7
  21:     0x7f6f9a0bc609 - start_thread
  22:     0x7f6f9f9cf133 - clone
  23:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (b11794869 2022-07-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/const-generics/mislabel-overlap-impl.rs stdout ----
error: test compilation failed although it shouldn't!
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/mislabel-overlap-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/mislabel-overlap-impl" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/mislabel-overlap-impl/auxiliary"
---
---- [ui] src/test/ui/const-generics/try_unify_ignore_lifetimes.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/try_unify_ignore_lifetimes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/try_unify_ignore_lifetimes" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/try_unify_ignore_lifetimes/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     fn baz(foo: &Foo<'a, Self::Size>) where [(); Self::Size::VALUE]: {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N::VALUE]:`
note: required by a bound in `Foo`
   |
   |
LL | struct Foo<'a, N: NumT>(&'a [u32; N::VALUE]) where [(); N::VALUE]:;
   |                                                         ^^^^^^^^ required by this bound in `Foo`
error: aborting due to previous error
------------------------------------------




failures:
    [ui] src/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical.rs
    [ui] src/test/ui/const-generics/generic_const_exprs/infer-too-generic.rs
    [ui] src/test/ui/const-generics/generic_const_exprs/issue-76595.rs
    [ui] src/test/ui/const-generics/generic_const_exprs/issue-97047-ice-1.rs
    [ui] src/test/ui/const-generics/generic_const_exprs/issue-97047-ice-2.rs
    [ui] src/test/ui/const-generics/mislabel-overlap-impl.rs

test result: FAILED. 13032 passed; 7 failed; 115 ignored; 0 measured; 0 filtered out; finished in 118.33s

Build completed unsuccessfully in 0:12:55
