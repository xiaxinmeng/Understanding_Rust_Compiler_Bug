console
$ cargo check
    Checking sadness v0.1.0 (file:///home/jon/d)
thread 'main' panicked at 'assertion failed: old_value.is_none()', librustc_mir/borrow_check/borrow_set.rs:350:9
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:211
   3: std::panicking::default_hook
             at libstd/panicking.rs:227
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:515
   6: std::panicking::begin_panic
   7: <rustc_mir::borrow_check::borrow_set::GatherBorrows<'a, 'gcx, 'tcx> as rustc::mir::visit::Visitor<'tcx>>::visit_assign
   8: rustc_mir::borrow_check::borrow_set::BorrowSet::build
   9: rustc_mir::borrow_check::do_mir_borrowck
  10: rustc::ty::context::tls::with_related_context
  11: rustc::infer::InferCtxtBuilder::enter
  12: rustc_mir::borrow_check::mir_borrowck
  13: rustc::ty::query::__query_compute::mir_borrowck
  14: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::mir_borrowck<'tcx>>::compute
  15: rustc::ty::context::tls::with_context
  16: rustc::dep_graph::graph::DepGraph::with_task_impl
  17: rustc::ty::context::tls::with_related_context
  18: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  19: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  20: rustc::ty::query::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::mir_borrowck
  21: rustc_mir::borrow_check::nll::type_check::TypeChecker::typeck_mir
  22: rustc_mir::borrow_check::nll::type_check::type_check_internal
  23: rustc_mir::borrow_check::nll::compute_regions
  24: rustc_mir::borrow_check::do_mir_borrowck
  25: rustc::ty::context::tls::with_related_context
  26: rustc::infer::InferCtxtBuilder::enter
  27: rustc_mir::borrow_check::mir_borrowck
  28: rustc::ty::query::__query_compute::mir_borrowck
  29: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::mir_borrowck<'tcx>>::compute
  30: rustc::ty::context::tls::with_context
  31: rustc::dep_graph::graph::DepGraph::with_task_impl
  32: rustc::ty::context::tls::with_related_context
  33: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  34: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  35: rustc::ty::query::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::mir_borrowck
  36: rustc::ty::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::par_body_owners
  37: rustc::util::common::time
  38: rustc::ty::context::tls::enter_context
  39: <std::thread::local::LocalKey<T>>::with
  40: rustc::ty::context::TyCtxt::create_and_enter
  41: rustc_driver::driver::compile_input
  42: rustc_driver::run_compiler_with_pool
  43: <scoped_tls::ScopedKey<T>>::set
  44: syntax::with_globals
  45: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  46: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:105
  47: rustc_driver::run
  48: rustc_driver::main
  49: std::rt::lang_start::{{closure}}
  50: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:310
  51: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:105
  52: std::rt::lang_start_internal
             at libstd/panicking.rs:289
             at libstd/panic.rs:397
             at libstd/rt.rs:58
  53: main
  54: __libc_start_main
  55: <unknown>
query stack during panic:
#0 [mir_borrowck] processing `sadness::{{closure}}`
#1 [mir_borrowck] processing `sadness`
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.28.0-nightly (2a1c4eec4 2018-06-25) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental -C target-cpu=native --crate-type lib

note: some of the compiler flags provided by cargo are hidden
