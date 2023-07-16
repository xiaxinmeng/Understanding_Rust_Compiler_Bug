
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at /checkout/src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/sys_common/backtrace.rs:60
             at /checkout/src/libstd/panicking.rs:381
   3: std::panicking::default_hook
             at /checkout/src/libstd/panicking.rs:391
   4: std::panicking::rust_panic_with_hook
             at /checkout/src/libstd/panicking.rs:577
   5: std::panicking::begin_panic
   6: rustc_errors::Handler::span_bug
   7: rustc::session::opt_span_bug_fmt::{{closure}}
   8: rustc::session::opt_span_bug_fmt
   9: rustc::session::span_bug_fmt
  10: rustc::infer::region_inference::RegionVarBindings::make_subregion
  11: rustc::infer::region_inference::RegionVarBindings::make_eqregion
  12: rustc::ty::relate::relate_substs::{{closure}}
  13: <core::result::Result<T, E> as rustc::ty::context::InternIteratorElement<T, R>>::intern_with
  14: rustc::infer::at::At::sub_exp
  15: rustc::traits::select::SelectionContext::match_projection
  16: rustc::traits::select::SelectionContext::match_projection_obligation_against_definition_bounds
  17: rustc::traits::select::SelectionContext::assemble_candidates
  18: rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache
  19: rustc::dep_graph::graph::DepGraph::with_anon_task
  20: rustc::traits::select::SelectionContext::candidate_from_obligation
  21: rustc::traits::select::SelectionContext::select
  22: <rustc::traits::fulfill::FulfillProcessor<'a, 'b, 'gcx, 'tcx> as rustc_data_structures::obligation_forest::ObligationProcessor>::process_obligation
  23: rustc::traits::fulfill::FulfillmentContext::select
  24: rustc::traits::fulfill::FulfillmentContext::select_where_possible
  25: rustc_typeck::check::FnCtxt::select_obligations_where_possible
  26: rustc_typeck::check::FnCtxt::check_argument_types
  27: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::confirm_builtin_call
  28: rustc_typeck::check::FnCtxt::check_expr_kind
  29: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref
  30: rustc_typeck::check::FnCtxt::check_block_with_expected::{{closure}}
  31: rustc_typeck::check::FnCtxt::check_block_with_expected
  32: rustc_typeck::check::FnCtxt::check_expr_kind
  33: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref
  34: rustc_typeck::check::FnCtxt::check_return_expr
  35: rustc_typeck::check::check_fn
  36: rustc_typeck::check::typeck_tables_of::{{closure}}
  37: rustc_typeck::check::typeck_tables_of
  38: rustc::dep_graph::graph::DepGraph::with_task
  39: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::try_get
  40: rustc::ty::maps::TyCtxtAt::typeck_tables_of
  41: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_tables_of
  42: rustc_typeck::check::typeck_item_bodies
  43: rustc::dep_graph::graph::DepGraph::with_task
  44: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::try_get
  45: rustc::ty::maps::TyCtxtAt::typeck_item_bodies
  46: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_item_bodies
  47: rustc_typeck::check_crate
  48: rustc::ty::context::TyCtxt::create_and_enter
  49: rustc_driver::driver::compile_input
  50: rustc_driver::run_compiler
