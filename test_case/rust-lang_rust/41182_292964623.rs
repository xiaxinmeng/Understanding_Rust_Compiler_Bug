
   0: rustc::session::opt_span_bug_fmt::{{closure}}
   1: rustc::session::span_bug_fmt
   2: rustc_typeck::check::FnCtxt::register_predicate
   3: <rustc::ty::fold::BottomUpFolder<'a, 'gcx, 'tcx, F> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
   4: rustc_typeck::check::check_fn
   5: rustc_typeck::check::typeck_tables
   6: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables<'tcx>>::try_get
   7: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables<'tcx>>::get
   8: rustc::ty::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::item_tables
   9: rustc_typeck::check::typeck_item_bodies
  10: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::try_get
  11: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::get
  12: rustc_typeck::check_crate
  13: rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}
  14: rustc::ty::context::TyCtxt::create_and_enter
  15: rustc_driver::driver::phase_3_run_analysis_passes
  16: rustc_driver::driver::compile_input
  17: rustc_driver::run_compiler
  18: std::panicking::try::do_call
  19: __rust_maybe_catch_panic
             at /checkout/src/libpanic_unwind/lib.rs:98
  20: <F as alloc::boxed::FnBox<A>>::call_box
  21: std::sys::imp::thread::Thread::new::thread_start
             at /checkout/src/liballoc/boxed.rs:650
             at /checkout/src/libstd/sys_common/thread.rs:21
             at /checkout/src/libstd/sys/unix/thread.rs:84
  22: start_thread
  23: clone
