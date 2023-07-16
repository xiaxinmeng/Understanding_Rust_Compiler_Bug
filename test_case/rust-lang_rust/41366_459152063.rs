
error[E0631]: type mismatch in closure arguments
 --> src/main.rs:8:5
  |
8 |     (&|_|()) as &for<'x> Fn(<u32 as T<'x>>::V);
  |     ^^-----^
  |     | |
  |     | found signature of `fn(u16) -> _`
  |     expected signature of `fn(<u32 as T<'x>>::V) -> _`
  |
  = note: required for the cast to the object type `dyn for<'x> std::ops::Fn(<u32 as T<'x>>::V)`

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
  13: rustc_typeck::check::writeback::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::resolve_type_vars_in_body
  14: rustc::ty::context::GlobalCtxt::enter_local
  15: rustc_typeck::check::typeck_tables_of
  16: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_tables_of<'tcx>>::compute
  17: rustc::dep_graph::graph::DepGraph::with_task_impl
  18: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_get_with
  19: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::ensure_query
  20: rustc::session::Session::track_errors
  21: rustc_typeck::check::typeck_item_bodies
  22: rustc::ty::query::__query_compute::typeck_item_bodies
  23: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_item_bodies<'tcx>>::compute
  24: rustc::dep_graph::graph::DepGraph::with_task_impl
  25: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_get_with
  26: rustc::util::common::time
  27: rustc_typeck::check_crate
  28: <std::thread::local::LocalKey<T>>::with
  29: rustc::ty::context::TyCtxt::create_and_enter
  30: rustc_driver::driver::compile_input
  31: rustc_driver::run_compiler_with_pool
  32: <scoped_tls::ScopedKey<T>>::set
  33: rustc_driver::run_compiler
  34: <scoped_tls::ScopedKey<T>>::set
query stack during panic:
#0 [typeck_tables_of] processing `main`
#1 [typeck_item_bodies] type-checking all item bodies
end of query stack
error: aborting due to previous error
