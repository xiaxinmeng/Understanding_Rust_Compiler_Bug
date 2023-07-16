
thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc_errors/lib.rs:375
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at /checkout/src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/sys_common/backtrace.rs:60
             at /checkout/src/libstd/panicking.rs:355
   3: std::panicking::default_hook
             at /checkout/src/libstd/panicking.rs:365
   4: std::panicking::rust_panic_with_hook
             at /checkout/src/libstd/panicking.rs:549
   5: std::panicking::begin_panic
   6: rustc::session::opt_span_bug_fmt::{{closure}}
   7: rustc::session::opt_span_bug_fmt
   8: rustc::session::span_bug_fmt
   9: rustc::traits::project::confirm_param_env_candidate
  10: rustc::traits::project::confirm_callable_candidate
  11: rustc::traits::project::confirm_closure_candidate
  12: rustc::traits::project::confirm_candidate
  13: rustc::traits::project::opt_normalize_projection_type
  14: rustc::traits::project::poly_project_and_unify_type::{{closure}}
  15: rustc::traits::project::poly_project_and_unify_type
  16: <rustc::traits::fulfill::FulfillProcessor<'a, 'b, 'gcx, 'tcx> as rustc_data_structures::obligation_forest::ObligationProcessor>::process_obligation
  17: <rustc_data_structures::obligation_forest::ObligationForest<O>>::process_obligations
  18: rustc::traits::fulfill::FulfillmentContext::select_where_possible
  19: rustc_typeck::check::FnCtxt::select_obligations_where_possible
  20: rustc_typeck::check::FnCtxt::check_argument_types
  21: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::confirm_builtin_call
  22: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::check_call
  23: rustc_typeck::check::FnCtxt::check_expr_kind
  24: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref
  25: rustc_typeck::check::FnCtxt::check_decl_initializer
  26: rustc_typeck::check::FnCtxt::check_decl_local
  27: rustc_typeck::check::FnCtxt::check_stmt
  28: rustc_typeck::check::FnCtxt::check_block_with_expected::{{closure}}
  29: rustc_typeck::check::FnCtxt::check_block_with_expected
  30: rustc_typeck::check::FnCtxt::check_expr_kind
  31: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref
  32: rustc_typeck::check::FnCtxt::check_stmt
  33: rustc_typeck::check::FnCtxt::check_block_with_expected::{{closure}}
  34: rustc_typeck::check::FnCtxt::check_block_with_expected
  35: rustc_typeck::check::FnCtxt::check_expr_kind
  36: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref
  37: rustc_typeck::check::FnCtxt::check_return_expr
  38: rustc_typeck::check::check_fn
  39: rustc_typeck::check::typeck_tables
  40: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables<'tcx>>::try_get
  41: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables<'tcx>>::get
  42: rustc::ty::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::item_tables
  43: rustc_typeck::check::typeck_item_bodies
  44: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::try_get
  45: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::get
  46: rustc_typeck::check_crate
  47: rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}
  48: rustc::ty::context::TyCtxt::create_and_enter
  49: rustc_driver::driver::phase_3_run_analysis_passes
  50: rustc_driver::driver::compile_input
  51: rustc_driver::run_compiler
  52: std::panicking::try::do_call
  53: __rust_maybe_catch_panic
             at /checkout/src/libpanic_unwind/lib.rs:98
  54: <F as alloc::boxed::FnBox<A>>::call_box
  55: std::sys::imp::thread::Thread::new::thread_start
             at /checkout/src/liballoc/boxed.rs:650
             at /checkout/src/libstd/sys_common/thread.rs:21
             at /checkout/src/libstd/sys/unix/thread.rs:84
  56: start_thread
  57: clone
