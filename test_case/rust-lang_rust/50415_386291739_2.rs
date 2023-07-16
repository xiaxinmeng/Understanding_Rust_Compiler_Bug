
error: internal compiler error: librustc/hir/map/mod.rs:329: local_def_id: no entry for `11`, which has a map of `Some(EntryTy(NodeId(14), 3, type(::std::ops::RangeInclusive)))`

thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:554:9
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::bug
   8: rustc::session::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::session::opt_span_bug_fmt
  13: rustc::session::bug_fmt
  14: rustc::hir::map::Map::local_def_id::{{closure}}
  15: rustc::hir::map::Map::body_owner_kind
  16: <rustc::middle::region::RegionResolutionVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_body
  17: rustc::middle::region::resolve_expr
  18: rustc::hir::intravisit::walk_stmt
  19: <rustc::middle::region::RegionResolutionVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_stmt
  20: <rustc::middle::region::RegionResolutionVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_block
  21: rustc::middle::region::resolve_expr
  22: <rustc::middle::region::RegionResolutionVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_body
  23: rustc::middle::region::region_scope_tree
  24: rustc::dep_graph::graph::DepGraph::with_task_impl
  25: rustc::ty::context::tls::with_related_context
  26: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  27: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  28: rustc_typeck::check::regionck::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::regionck_item
  29: rustc::ty::context::tls::with_related_context
  30: rustc::infer::InferCtxtBuilder::enter
  31: rustc_typeck::check::wfcheck::check_item_well_formed
  32: rustc::dep_graph::graph::DepGraph::with_task_impl
  33: rustc::ty::context::tls::with_related_context
  34: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  35: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  36: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::ensure_query
  37: rustc::hir::Crate::visit_all_item_likes
  38: rustc::util::common::time
  39: rustc_typeck::check_crate
  40: rustc::ty::context::tls::enter_context
  41: <std::thread::local::LocalKey<T>>::with
  42: rustc::ty::context::TyCtxt::create_and_enter
  43: rustc_driver::driver::compile_input
  44: rustc_driver::run_compiler_impl
  45: <scoped_tls::ScopedKey<T>>::set
  46: syntax::with_globals
  47: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  48: __rust_maybe_catch_panic
  49: rustc_driver::run
  50: rustc_driver::main
  51: std::rt::lang_start::{{closure}}
  52: std::panicking::try::do_call
  53: __rust_maybe_catch_panic
  54: std::rt::lang_start_internal
  55: main
query stack during panic:
#0 [region_scope_tree] processing `main`
#1 [check_item_well_formed] processing `main`
end of query stack
