
   0: rust_begin_unwind
             at /rustc/98c8619502093f34ca82f8f26ccf32e753924440/library/std/src/panicking.rs:517:5
   1: core::panicking::panic_fmt
             at /rustc/98c8619502093f34ca82f8f26ccf32e753924440/library/core/src/panicking.rs:100:14
   2: rustc_errors::HandlerInner::emit_diagnostic
   3: rustc_errors::diagnostic_builder::DiagnosticBuilder::emit
   4: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_overflow_error
   5: rustc_trait_selection::traits::select::SelectionContext::evaluate_predicates_recursively
   6: rustc_infer::infer::InferCtxt::probe
   7: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
   8: rustc_trait_selection::traits::select::candidate_assembly::<impl rustc_trait_selection::traits::select::SelectionContext>::candidate_from_obligation_no_cache
   9: rustc_trait_selection::traits::select::SelectionContext::select
  10: rustc_trait_selection::traits::project::opt_normalize_projection_type
  11: rustc_trait_selection::traits::project::poly_project_and_unify_type
  12: rustc_trait_selection::traits::select::SelectionContext::evaluate_predicates_recursively
  13: rustc_infer::infer::InferCtxt::probe
  14: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
  15: rustc_trait_selection::traits::select::candidate_assembly::<impl rustc_trait_selection::traits::select::SelectionContext>::candidate_from_obligation_no_cache
  16: rustc_trait_selection::traits::select::SelectionContext::select
  17: rustc_trait_selection::traits::project::opt_normalize_projection_type
  18: rustc_trait_selection::traits::project::poly_project_and_unify_type
  19: rustc_trait_selection::traits::select::SelectionContext::evaluate_predicates_recursively
  20: rustc_infer::infer::InferCtxt::probe
  21: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
  22: rustc_trait_selection::traits::select::candidate_assembly::<impl rustc_trait_selection::traits::select::SelectionContext>::candidate_from_obligation_no_cache
  23: rustc_trait_selection::traits::select::SelectionContext::select
  24: rustc_trait_selection::traits::project::opt_normalize_projection_type
  25: rustc_trait_selection::traits::project::poly_project_and_unify_type
  26: rustc_trait_selection::traits::select::SelectionContext::evaluate_predicates_recursively
  27: rustc_infer::infer::InferCtxt::probe
  28: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
  29: rustc_trait_selection::traits::select::candidate_assembly::<impl rustc_trait_selection::traits::select::SelectionContext>::candidate_from_obligation_no_cache
  30: rustc_trait_selection::traits::select::SelectionContext::select
  31: rustc_trait_selection::traits::project::opt_normalize_projection_type
  32: rustc_trait_selection::traits::project::poly_project_and_unify_type
  33: rustc_trait_selection::traits::select::SelectionContext::evaluate_predicates_recursively
  34: rustc_infer::infer::InferCtxt::probe
  35: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
  36: rustc_trait_selection::traits::select::candidate_assembly::<impl rustc_trait_selection::traits::select::SelectionContext>::candidate_from_obligation_no_cache
  37: rustc_trait_selection::traits::select::SelectionContext::select
  38: rustc_trait_selection::traits::project::opt_normalize_projection_type
  39: rustc_trait_selection::traits::project::poly_project_and_unify_type
  40: rustc_trait_selection::traits::select::SelectionContext::evaluate_predicates_recursively
  41: rustc_infer::infer::InferCtxt::probe
  42: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
  43: rustc_trait_selection::traits::select::candidate_assembly::<impl rustc_trait_selection::traits::select::SelectionContext>::candidate_from_obligation_no_cache
  44: rustc_trait_selection::traits::select::SelectionContext::select
  45: rustc_trait_selection::traits::project::opt_normalize_projection_type
  46: rustc_trait_selection::traits::project::poly_project_and_unify_type
  47: rustc_trait_selection::traits::select::SelectionContext::evaluate_predicates_recursively
  48: rustc_infer::infer::InferCtxt::probe
  49: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
  50: rustc_trait_selection::traits::select::candidate_assembly::<impl rustc_trait_selection::traits::select::SelectionContext>::candidate_from_obligation_no_cache
  51: rustc_trait_selection::traits::select::SelectionContext::select
  52: rustc_trait_selection::traits::project::opt_normalize_projection_type
  53: rustc_trait_selection::traits::project::poly_project_and_unify_type
  54: rustc_trait_selection::traits::select::SelectionContext::evaluate_predicates_recursively
  55: rustc_infer::infer::InferCtxt::probe
  56: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
  57: rustc_trait_selection::traits::select::candidate_assembly::<impl rustc_trait_selection::traits::select::SelectionContext>::candidate_from_obligation_no_cache
  58: rustc_trait_selection::traits::select::SelectionContext::select
  59: rustc_trait_selection::traits::project::opt_normalize_projection_type
  60: rustc_trait_selection::traits::project::poly_project_and_unify_type
  61: rustc_trait_selection::traits::select::SelectionContext::evaluate_predicates_recursively
  62: rustc_infer::infer::InferCtxt::probe
  63: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
  64: rustc_trait_selection::traits::select::candidate_assembly::<impl rustc_trait_selection::traits::select::SelectionContext>::candidate_from_obligation_no_cache
  65: rustc_query_system::dep_graph::graph::DepGraph<K>::with_anon_task
  66: rustc_trait_selection::traits::select::SelectionContext::evaluate_stack
  67: rustc_trait_selection::traits::select::SelectionContext::evaluate_trait_predicate_recursively
  68: rustc_trait_selection::traits::select::SelectionContext::evaluate_root_obligation
  69: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::evaluate_obligation::InferCtxtExt>::predicate_may_hold
  70: rustc_infer::infer::InferCtxt::probe
  71: rustc_typeck::check::method::probe::ProbeContext::pick_method
  72: rustc_typeck::check::method::probe::ProbeContext::pick_core
  73: rustc_typeck::check::method::probe::ProbeContext::pick
  74: rustc_infer::infer::InferCtxt::probe
  75: rustc_typeck::check::method::probe::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::probe_op
  76: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once
  77: <core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next
  78: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
  79: rustc_typeck::check::method::probe::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::probe_for_return_type
  80: rustc_typeck::check::demand::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::get_conversion_methods
  81: rustc_typeck::check::fn_ctxt::suggestions::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::suggest_deref_ref_or_into
  82: rustc_typeck::check::demand::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::emit_coerce_suggestions
  83: rustc_typeck::check::demand::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::demand_coerce
  84: rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_decl_local
  85: rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_stmt
  86: rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_block_with_expected
  87: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  88: rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_return_expr
  89: rustc_typeck::check::check::check_fn
  90: rustc_infer::infer::InferCtxtBuilder::enter
  91: rustc_typeck::check::typeck
