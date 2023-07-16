
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.25.0-nightly (a0dcecff9 2018-01-24) running on x86_64-unknown-linux-gnu

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', libcore/option.rs:335:21
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:68
             at libstd/sys_common/backtrace.rs:57
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:380
   3: std::panicking::default_hook
             at libstd/panicking.rs:390
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
  10: rustc::middle::region::ScopeTree::yield_in_scope_for_expr
  11: <rustc_borrowck::borrowck::check_loans::CheckLoanCtxt<'a, 'tcx> as rustc::middle::expr_use_visitor::Delegate<'tcx>>::borrow
  12: rustc::middle::mem_categorization::MemCategorizationContext::cat_pattern_
  13: rustc::middle::mem_categorization::MemCategorizationContext::cat_pattern_
  14: rustc::middle::expr_use_visitor::ExprUseVisitor::walk_pat
  15: rustc::middle::expr_use_visitor::ExprUseVisitor::walk_expr
  16: rustc::middle::expr_use_visitor::ExprUseVisitor::walk_local
  17: rustc::middle::expr_use_visitor::ExprUseVisitor::walk_expr
  18: rustc::middle::expr_use_visitor::ExprUseVisitor::walk_expr
  19: rustc::middle::expr_use_visitor::ExprUseVisitor::walk_local
  20: rustc::middle::expr_use_visitor::ExprUseVisitor::walk_expr
  21: rustc_borrowck::borrowck::check_loans::check_loans
  22: rustc_borrowck::borrowck::borrowck
  23: rustc::dep_graph::graph::DepGraph::with_task_impl
  24: rustc_errors::Handler::track_diagnostics
  25: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
  26: rustc::ty::maps::<impl rustc::ty::maps::queries::borrowck<'tcx>>::force
  27: rustc::ty::maps::<impl rustc::ty::maps::queries::borrowck<'tcx>>::try_get
  28: rustc::ty::maps::TyCtxtAt::borrowck
  29: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::borrowck
  30: rustc_borrowck::borrowck::check_crate
  31: <std::thread::local::LocalKey<T>>::with
  32: <std::thread::local::LocalKey<T>>::with
  33: rustc_driver::driver::compile_input
  34: rustc_driver::run_compiler
