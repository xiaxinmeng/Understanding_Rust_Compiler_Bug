
thread 'main' panicked at 'region_obligations not empty: [
    (
        NodeId(
            22
        ),
        RegionObligation(sub_region='_#1r, sup_type=T)
    )
]', librustc/infer/mod.rs:1057:9
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::continue_panic_fmt
   7: std::panicking::begin_panic_fmt
   8: rustc::infer::InferCtxt::take_and_reset_region_constraints
   9: rustc_mir::borrow_check::nll::type_check::TypeChecker::sub_types
  10: rustc_mir::borrow_check::nll::type_check::TypeChecker::typeck_mir
  11: rustc_mir::borrow_check::nll::type_check::type_check_internal
  12: rustc_mir::borrow_check::nll::compute_regions
  13: rustc_mir::borrow_check::do_mir_borrowck
  14: rustc::ty::context::GlobalCtxt::enter_local
  15: rustc_mir::borrow_check::mir_borrowck
  16: rustc::ty::query::__query_compute::mir_borrowck
  17: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::mir_borrowck<'tcx>>::compute
  18: rustc::ty::context::tls::with_context
  19: rustc::dep_graph::graph::DepGraph::with_task_impl
  20: rustc::ty::context::tls::with_related_context
  21: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  22: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  23: rustc::ty::query::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::mir_borrowck
  24: rustc::ty::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::par_body_owners
  25: rustc::util::common::time
  26: rustc::ty::context::tls::enter_context
  27: <std::thread::local::LocalKey<T>>::with
  28: rustc::ty::context::TyCtxt::create_and_enter
  29: rustc_driver::driver::compile_input
  30: rustc_driver::run_compiler_with_pool
  31: <scoped_tls::ScopedKey<T>>::set
  32: syntax::with_globals
  33: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  34: __rust_maybe_catch_panic
  35: rustc_driver::run
  36: rustc_driver::main
  37: std::rt::lang_start::{{closure}}
  38: std::panicking::try::do_call
  39: __rust_maybe_catch_panic
  40: std::rt::lang_start_internal
  41: main
query stack during panic:
#0 [mir_borrowck] processing `<A<T> as std::ops::Deref>::deref`
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.28.0-nightly (f28c7aef7 2018-06-19) running on x86_64-apple-darwin

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden
