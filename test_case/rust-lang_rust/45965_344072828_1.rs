
error[E0618]: expected function, found `{float}`
 --> test.rs:2:30
  |
2 |     let a = |r: f64| if r != 0.0(r != 0.0) {1.0} else {0.0};
  |                              ^^^^^^^^^^^^^

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.21.0 (3b72af97e 2017-10-09) running on x86_64-unknown-linux-gnu

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'LocalTableInContext: key not found', /checkout/src/libcore/option.rs:819:4
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
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
             at /checkout/src/libstd/panicking.rs:611
   5: std::panicking::begin_panic
             at /checkout/src/libstd/panicking.rs:572
   6: std::panicking::begin_panic_fmt
             at /checkout/src/libstd/panicking.rs:522
   7: rust_begin_unwind
             at /checkout/src/libstd/panicking.rs:498
   8: core::panicking::panic_fmt
             at /checkout/src/libcore/panicking.rs:71
   9: core::option::expect_failed
             at /checkout/src/libcore/option.rs:819
  10: rustc::middle::expr_use_visitor::ExprUseVisitor::walk_expr
  11: rustc::middle::expr_use_visitor::ExprUseVisitor::consume_expr
  12: rustc::middle::expr_use_visitor::ExprUseVisitor::walk_expr
  13: rustc::middle::expr_use_visitor::ExprUseVisitor::consume_expr
  14: rustc::middle::expr_use_visitor::ExprUseVisitor::walk_expr
  15: rustc::middle::expr_use_visitor::ExprUseVisitor::consume_expr
  16: rustc::middle::expr_use_visitor::ExprUseVisitor::consume_body
  17: <rustc_typeck::check::upvar::InferBorrowKindVisitor<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr
  18: rustc::hir::intravisit::walk_block
  19: <rustc_typeck::check::upvar::InferBorrowKindVisitor<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr
  20: rustc_typeck::check::typeck_tables_of::{{closure}}
  21: rustc_typeck::check::typeck_tables_of
  22: rustc::dep_graph::graph::DepGraph::with_task
  23: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::try_get
  24: rustc::ty::maps::TyCtxtAt::typeck_tables_of
  25: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_tables_of
  26: rustc_typeck::check::typeck_item_bodies
  27: rustc::dep_graph::graph::DepGraph::with_task
  28: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::try_get
  29: rustc::ty::maps::TyCtxtAt::typeck_item_bodies
  30: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_item_bodies
  31: rustc_typeck::check_crate
  32: rustc::ty::context::TyCtxt::create_and_enter
  33: rustc_driver::driver::compile_input
  34: rustc_driver::run_compiler
