
stack backtrace:
   7: rustc::ty::util::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::dtorck_constraint_for_ty
   8: rustc_typeck::check::dropck::check_safety_of_destructor_if_necessary
   9: rustc_typeck::check::regionck::RegionCtxt::constrain_bindings_in_pat::{{closure}}
  10: rustc::hir::pat_util::<impl rustc::hir::Pat>::each_binding::{{closure}}
  11: rustc::hir::Pat::walk_
  12: rustc::hir::Pat::walk
  13: rustc::hir::pat_util::<impl rustc::hir::Pat>::each_binding
  14: rustc_typeck::check::regionck::RegionCtxt::constrain_bindings_in_pat
  15: <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_local
  16: rustc::hir::intravisit::walk_block
  17: rustc::hir::intravisit::walk_expr
  18: <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr
  19: rustc_typeck::check::regionck::RegionCtxt::visit_fn_body
  20: <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_fn
  21: rustc::hir::intravisit::walk_expr
  22: rustc_typeck::check::regionck::RegionCtxt::check_expr_fn_block
  23: <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr
  24: rustc::hir::intravisit::walk_expr
  25: <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr
  26: rustc::hir::intravisit::walk_local
  27: rustc::hir::intravisit::walk_block
  28: rustc::hir::intravisit::walk_expr
  29: <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr
  30: rustc_typeck::check::regionck::RegionCtxt::visit_fn_body
  31: rustc_typeck::check::regionck::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::regionck_fn
  32: rustc_typeck::check::typeck_tables_of::{{closure}}
  33: rustc_typeck::check::InheritedBuilder::enter::{{closure}}
  34: rustc::infer::InferCtxtBuilder::enter::{{closure}}
  35: rustc::ty::context::tls::enter::{{closure}}
  36: <std::thread::local::LocalKey<T>>::with
  37: rustc::ty::context::tls::enter
  38: rustc::ty::context::GlobalCtxt::enter_local
  39: rustc::infer::InferCtxtBuilder::enter
  40: rustc_typeck::check::InheritedBuilder::enter
  41: rustc_typeck::check::typeck_tables_of
  42: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::try_get_with::{{closure}}
  43: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
  44: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::try_get_with
  45: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::try_get
  46: rustc::ty::maps::TyCtxtAt::typeck_tables_of
  47: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_tables_of
  48: rustc_typeck::check::typeck_item_bodies::{{closure}}
  49: rustc::session::Session::track_errors
  50: rustc_typeck::check::typeck_item_bodies
  51: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::try_get_with::{{closure}}
  52: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
  53: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::try_get_with
  54: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::try_get
  55: rustc::ty::maps::TyCtxtAt::typeck_item_bodies
  56: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_item_bodies
  57: rustc_typeck::check::check_item_bodies
  58: rustc_typeck::check_crate::{{closure}}
  59: rustc::util::common::time
  60: rustc_typeck::check_crate
  61: rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}
  62: rustc::ty::context::tls::enter::{{closure}}
  63: <std::thread::local::LocalKey<T>>::with
  64: rustc::ty::context::tls::enter
  65: rustc::ty::context::tls::enter_global::{{closure}}
  66: <std::thread::local::LocalKey<T>>::with
  67: rustc::ty::context::tls::enter_global
  68: rustc::ty::context::TyCtxt::create_and_enter
  69: rustc_driver::driver::phase_3_run_analysis_passes
  70: rustc_driver::driver::compile_input
  71: rustc_driver::run_compiler
  72: rustc_driver::main::{{closure}}
  73: rustc_driver::run::{{closure}}
  74: rustc_driver::monitor::{{closure}}
  75: __rust_try
  76: std::panicking::try
  77: std::panicking::try
  78: std::panic::catch_unwind
  79: std::thread::Builder::spawn::{{closure}}
  80: <F as alloc::boxed::FnBox<A>>::call_box
  81: std::sys_common::thread::start_thread
  82: std::sys::imp::thread::Thread::new::thread_start
  83: _pthread_body
  84: _pthread_start
