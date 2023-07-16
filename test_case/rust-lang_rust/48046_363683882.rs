
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:380
   3: std::panicking::default_hook
             at libstd/panicking.rs:396
   4: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:576
   5: std::panicking::begin_panic
             at libstd/panicking.rs:537
   6: std::panicking::begin_panic_fmt
             at libstd/panicking.rs:521
   7: rust_begin_unwind
             at libstd/panicking.rs:497
   8: core::panicking::panic_fmt
             at libcore/panicking.rs:71
   9: core::panicking::panic
             at libcore/panicking.rs:51
  10: rustc::traits::error_reporting::<impl rustc::infer::InferCtxt<'a, 'gcx, 'tcx>>::report_selection_error
  11: rustc::traits::error_reporting::<impl rustc::infer::InferCtxt<'a, 'gcx, 'tcx>>::report_fulfillment_errors
  12: rustc_typeck::check::FnCtxt::select_obligations_where_possible
  13: rustc_typeck::check::demand::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::demand_coerce_diag
  14: rustc_typeck::check::demand::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::demand_coerce
  15: rustc_typeck::check::FnCtxt::check_decl_local
  16: rustc_typeck::check::FnCtxt::check_block_with_expected
  17: rustc_typeck::check::FnCtxt::check_expr_kind
  18: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref
  19: rustc_typeck::check::FnCtxt::check_return_expr
  20: rustc_typeck::check::check_fn
  21: <std::thread::local::LocalKey<T>>::with
  22: rustc::ty::context::GlobalCtxt::enter_local
  23: rustc_typeck::check::typeck_tables_of
  24: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::compute_result
  25: rustc::dep_graph::graph::DepGraph::with_task_impl
  26: rustc_errors::Handler::track_diagnostics
  27: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
  28: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::force
  29: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::try_get
  30: rustc::ty::maps::TyCtxtAt::typeck_tables_of
  31: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::ensure
  32: rustc::session::Session::track_errors
  33: rustc_typeck::check::typeck_item_bodies
  34: rustc::dep_graph::graph::DepGraph::with_task_impl
  35: rustc_errors::Handler::track_diagnostics
  36: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
  37: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::force
  38: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::try_get
  39: rustc::ty::maps::TyCtxtAt::typeck_item_bodies
  40: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_item_bodies
  41: rustc_typeck::check_crate
  42: <std::thread::local::LocalKey<T>>::with
  43: <std::thread::local::LocalKey<T>>::with
  44: rustc::ty::context::TyCtxt::create_and_enter
  45: rustc_driver::driver::compile_input
  46: rustc_driver::run_compiler
