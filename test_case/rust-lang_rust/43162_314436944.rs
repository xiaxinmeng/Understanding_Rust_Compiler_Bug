
thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc_errors/lib.rs:480
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
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
   6: rustc_errors::Handler::bug
   7: rustc::session::opt_span_bug_fmt::{{closure}}
   8: rustc::session::opt_span_bug_fmt
   9: rustc::session::bug_fmt
  10: rustc_typeck::check::FnCtxt::node_ty
  11: <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr
  12: <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr
  13: rustc::hir::intravisit::walk_expr
  14: <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr
  15: rustc_typeck::check::regionck::RegionCtxt::visit_fn_body
  16: rustc_typeck::check::typeck_tables_of::{{closure}}
  17: rustc_typeck::check::typeck_tables_of
  18: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::try_get
  19: rustc::ty::maps::TyCtxtAt::typeck_tables_of
  20: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_tables_of
  21: rustc_typeck::check::typeck_item_bodies
  22: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::try_get
  23: rustc::ty::maps::TyCtxtAt::typeck_item_bodies
  24: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_item_bodies
  25: rustc_typeck::check_crate
  26: rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}
  27: rustc_driver::driver::phase_3_run_analysis_passes
  28: rustc_driver::driver::compile_input
  29: rustc_driver::run_compiler
