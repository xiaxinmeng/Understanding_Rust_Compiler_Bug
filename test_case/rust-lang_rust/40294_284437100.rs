
rustc 1.17.0-nightly (b1e31766d 2017-03-03)
error: internal compiler error: /checkout/src/librustc/hir/map/mod.rs:722: expected expr, found fn zip (id=4)

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc_errors/lib.rs:417
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: rustc_errors::Handler::bug
   1: rustc::session::opt_span_bug_fmt::{{closure}}
   2: rustc::session::opt_span_bug_fmt
   3: rustc::session::bug_fmt
   4: rustc::hir::map::Map::expect_expr
   5: rustc::traits::error_reporting::<impl rustc::infer::InferCtxt<'a, 'gcx, 'tcx>>::need_type_info
   6: rustc::traits::error_reporting::<impl rustc::infer::InferCtxt<'a, 'gcx, 'tcx>>::report_fulfillment_error
   7: rustc::traits::error_reporting::<impl rustc::infer::InferCtxt<'a, 'gcx, 'tcx>>::report_fulfillment_errors
   8: rustc_typeck::check::FnCtxt::select_all_obligations_or_error
   9: <std::thread::local::LocalKey<T>>::with
  10: rustc_typeck::check::wfcheck::CheckTypeWellFormedVisitor::check_item_well_formed
  11: rustc_typeck::check::check_wf_new
  12: rustc_typeck::check_crate
  13: rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}
  14: rustc_driver::driver::phase_3_run_analysis_passes
  15: rustc_driver::driver::compile_input
  16: rustc_driver::run_compiler
  17: std::panicking::try::do_call
  18: __rust_maybe_catch_panic
             at /checkout/src/libpanic_unwind/lib.rs:98
  19: <F as alloc::boxed::FnBox<A>>::call_box
  20: std::sys::imp::thread::Thread::new::thread_start
             at /checkout/src/liballoc/boxed.rs:648
             at /checkout/src/libstd/sys_common/thread.rs:21
             at /checkout/src/libstd/sys/unix/thread.rs:84
  21: start_thread
  22: clone
