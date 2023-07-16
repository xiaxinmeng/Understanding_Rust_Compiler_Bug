
error: internal compiler error: src/librustc_mir/borrow_check/nll/mod.rs:354: region is not an ReVar: ReStatic                                                              

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:634:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::bug
   8: rustc::util::bug::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::util::bug::opt_span_bug_fmt
  13: rustc::util::bug::bug_fmt
  14: rustc_mir::borrow_check::nll::type_check::TypeChecker::check_rvalue
  15: rustc_mir::borrow_check::nll::type_check::TypeChecker::typeck_mir
  16: rustc_mir::borrow_check::nll::type_check::type_check_internal
  17: rustc_mir::borrow_check::nll::type_check::type_check
  18: rustc_mir::borrow_check::nll::compute_regions
  19: rustc_mir::borrow_check::do_mir_borrowck
  20: rustc::ty::context::GlobalCtxt::enter_local
  21: rustc_mir::borrow_check::mir_borrowck
  22: rustc::ty::query::__query_compute::mir_borrowck
  23: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::mir_borrowck<'tcx>>::compute
  24: rustc::dep_graph::graph::DepGraph::with_task_impl
  25: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  26: rustc::ty::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::par_body_owners
  27: rustc::util::common::time
  28: rustc_interface::passes::analysis
  29: rustc::ty::query::__query_compute::analysis
  30: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::analysis<'tcx>>::compute
  31: rustc::dep_graph::graph::DepGraph::with_task_impl
  32: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  33: rustc::ty::context::tls::enter_global
  34: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  35: rustc_interface::passes::create_global_ctxt::{{closure}}
  36: rustc_interface::passes::BoxedGlobalCtxt::enter
  37: rustc_interface::interface::run_compiler_in_existing_thread_pool
  38: <std::thread::local::LocalKey<T>>::with
  39: <scoped_tls::ScopedKey<T>>::set
  40: syntax::with_globals
query stack during panic:
#0 [mir_borrowck] processing `fmt::num::imp::fmt_u64`
#1 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error
