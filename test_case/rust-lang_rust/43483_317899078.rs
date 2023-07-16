
error: internal compiler error: /checkout/src/librustc_const_eval/eval.rs:552: resolve_trait_associated_const: unexpected vtable type                                                                                                        
                                                                                                                                                                                                                                             
note: the compiler unexpectedly panicked. this is a bug.                                                                                                                                                                                     

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.20.0-nightly (598eddf4f 2017-07-24) running on x86_64-unknown-linux-gnu

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc_errors/lib.rs:490:8
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at /checkout/src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/sys_common/backtrace.rs:60
             at /checkout/src/libstd/panicking.rs:380
   3: std::panicking::default_hook
             at /checkout/src/libstd/panicking.rs:390
   4: std::panicking::rust_panic_with_hook
             at /checkout/src/libstd/panicking.rs:611
   5: std::panicking::begin_panic_new
   6: rustc_errors::Handler::bug
   7: rustc::session::opt_span_bug_fmt::{{closure}}
   8: rustc::session::opt_span_bug_fmt
   9: rustc::session::bug_fmt
  10: rustc_const_eval::eval::resolve_trait_associated_const
  11: rustc_const_eval::eval::lookup_const_by_id
  12: rustc_const_eval::eval::const_eval
  13: rustc::dep_graph::graph::DepGraph::with_task
  14: rustc::ty::maps::<impl rustc::ty::maps::queries::const_eval<'tcx>>::try_get
  15: rustc::ty::maps::TyCtxtAt::const_eval
  16: rustc_const_eval::eval::eval_const_expr_partial
  17: rustc_const_eval::eval::ConstContext::eval
  18: <rustc_passes::consts::CheckCrateVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_expr
  19: <rustc_passes::consts::CheckCrateVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_expr
  20: <rustc_passes::consts::CheckCrateVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_nested_body
  21: rustc_passes::consts::check_crate
  22: rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}
  23: rustc_driver::driver::phase_3_run_analysis_passes
  24: rustc_driver::driver::compile_input
  25: rustc_driver::run_compiler
