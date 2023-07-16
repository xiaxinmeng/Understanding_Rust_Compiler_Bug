
error[E0601]: main function not found

error[E0268]: `break` outside of loop
 --> 1.rs:2:5
  |
2 |     break true;
  |     ^^^^^^^^^^ cannot break outside of a loop

error: internal compiler error: src/librustc_typeck/check/mod.rs:1988: no type for node 8: expr true (id=8) in fcx 0x700006ca1a38

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.21.0-nightly (ed16b0a1d 2017-08-05) running on x86_64-apple-darwin

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:486:8
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
   1: std::panicking::default_hook::{{closure}}
   2: std::panicking::default_hook
   3: std::panicking::rust_panic_with_hook
   4: std::panicking::begin_panic
   5: rustc_errors::Handler::bug
   6: rustc::session::opt_span_bug_fmt::{{closure}}
   7: rustc::session::opt_span_bug_fmt
   8: rustc::session::bug_fmt
   9: rustc_typeck::check::FnCtxt::node_ty
  10: <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr
  11: <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr
  12: rustc::hir::intravisit::walk_expr
  13: <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr
  14: rustc_typeck::check::regionck::RegionCtxt::visit_fn_body
  15: rustc_typeck::check::typeck_tables_of::{{closure}}
  16: rustc_typeck::check::typeck_tables_of
  17: rustc::dep_graph::graph::DepGraph::with_task
  18: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::try_get
  19: rustc::ty::maps::TyCtxtAt::typeck_tables_of
  20: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_tables_of
  21: rustc_typeck::check::typeck_item_bodies
  22: rustc::dep_graph::graph::DepGraph::with_task
  23: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::try_get
  24: rustc::ty::maps::TyCtxtAt::typeck_item_bodies
  25: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_item_bodies
  26: rustc_typeck::check_crate
  27: rustc::ty::context::TyCtxt::create_and_enter
  28: rustc_driver::driver::compile_input
  29: rustc_driver::run_compiler
