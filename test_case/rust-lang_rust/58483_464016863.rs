none
thread 'rustc' panicked at 'assertion failed: !ty.needs_infer() && !ty.has_placeholders()', src/librustc_typeck/check/writeback.rs:119:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
   1: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:70
   2: std::panicking::default_hook::{{closure}}
             at src/libstd/sys_common/backtrace.rs:58
             at src/libstd/panicking.rs:200
   3: std::panicking::default_hook
             at src/libstd/panicking.rs:215
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:482
   6: std::panicking::begin_panic
   7: rustc_typeck::check::writeback::WritebackCx::visit_node_id
   8: <rustc_typeck::check::writeback::WritebackCx<'cx, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr
   9: rustc::hir::intravisit::walk_expr
  10: <rustc_typeck::check::writeback::WritebackCx<'cx, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr
  11: rustc::hir::intravisit::walk_expr
  12: <rustc_typeck::check::writeback::WritebackCx<'cx, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr
  13: <rustc_typeck::check::writeback::WritebackCx<'cx, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_local
  14: rustc::hir::intravisit::walk_expr
  15: <rustc_typeck::check::writeback::WritebackCx<'cx, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr
  16: rustc_typeck::check::writeback::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::resolve_type_vars_in_body
  17: rustc::ty::context::GlobalCtxt::enter_local
  18: rustc_typeck::check::typeck_tables_of
  19: rustc::ty::query::__query_compute::typeck_tables_of
  20: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_tables_of<'tcx>>::compute
  21: rustc::dep_graph::graph::DepGraph::with_task_impl
  22: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_get_with
  23: rustc::ty::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::par_body_owners
  24: rustc_typeck::check::typeck_item_bodies
  25: rustc::ty::query::__query_compute::typeck_item_bodies
  26: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_item_bodies<'tcx>>::compute
  27: rustc::dep_graph::graph::DepGraph::with_task_impl
  28: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_get_with
  29: rustc::util::common::time
  30: rustc_typeck::check_crate
  31: <std::thread::local::LocalKey<T>>::with
  32: rustc::ty::context::TyCtxt::create_and_enter
  33: rustc_driver::driver::compile_input
  34: rustc_driver::run_compiler_with_pool
  35: <scoped_tls::ScopedKey<T>>::set
  36: rustc_driver::run_compiler
  37: <scoped_tls::ScopedKey<T>>::set
query stack during panic:
#0 [typeck_tables_of] processing `main`
#1 [typeck_item_bodies] type-checking all item bodies
end of query stack
error: aborting due to 2 previous errors

Some errors occurred: E0277, E0599.
For more information about an error, try `rustc --explain E0277`.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.34.0-nightly (f47ec2ad5 2019-02-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin
