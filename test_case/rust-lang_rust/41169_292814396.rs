
error: internal compiler error: src/librustc/hir/map/mod.rs:859: unexpected expr ExprClosure(CaptureByValue, FnDecl { inputs: [type(_), type(_)], output: DefaultReturn(src/libcore/iter/mod.rs:421:44: 421:44), variadic: false, has_implicit_self: false }, BodyId { node_id: NodeId(25707) }, src/libcore/iter/mod.rs:421:28: 421:43)
   --> src/libcore/iter/mod.rs:421:28
    |
421 |         self.it.fold(init, move |acc, elt| f(acc, elt.clone()))
    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:375
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: rustc::session::opt_span_bug_fmt::{{closure}}
   1: rustc::session::opt_span_bug_fmt
   2: rustc::session::span_bug_fmt
   3: rustc::hir::map::Map::get_pattern_source
   4: <rustc_borrowck::borrowck::gather_loans::GatherLoanCtxt<'a, 'tcx> as rustc::middle::expr_use_visitor::Delegate<'tcx>>::consume_pat
   5: rustc::middle::mem_categorization::MemCategorizationContext::cat_pattern_
   6: rustc::middle::expr_use_visitor::ExprUseVisitor::walk_pat
   7: rustc::middle::expr_use_visitor::ExprUseVisitor::consume_body
   8: rustc_borrowck::borrowck::build_borrowck_dataflow_data
   9: rustc_borrowck::borrowck::check_crate
  10: rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}
  11: rustc_driver::driver::phase_3_run_analysis_passes
  12: rustc_driver::driver::compile_input
  13: rustc_driver::run_compiler
  14: std::panicking::try::do_call
  15: __rust_maybe_catch_panic
  16: <F as alloc::boxed::FnBox<A>>::call_box
  17: std::sys::imp::thread::Thread::new::thread_start
  18: start_thread
  19: clone

error: Could not compile `core`.

