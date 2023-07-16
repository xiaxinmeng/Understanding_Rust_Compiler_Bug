
thread 'main' panicked at 'region_obligations not empty: [
    (
        NodeId(
            3053
        ),
        RegionObligation(sub_region=ReStatic, sup_type=A)
    )
]', librustc/infer/mod.rs:1057:9
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
   6: std::panicking::continue_panic_fmt
             at libstd/panicking.rs:426
   7: std::panicking::begin_panic_fmt
             at libstd/panicking.rs:413
   8: rustc::infer::InferCtxt::take_and_reset_region_constraints
   9: rustc_mir::borrow_check::nll::type_check::TypeChecker::normalize
  10: <rustc_mir::borrow_check::nll::type_check::TypeVerifier<'a, 'b, 'gcx, 'tcx> as rustc::mir::visit::Visitor<'tcx>>::visit_constant
  11: <rustc_mir::borrow_check::nll::type_check::TypeVerifier<'a, 'b, 'gcx, 'tcx> as rustc::mir::visit::Visitor<'tcx>>::visit_mir
  12: rustc_mir::borrow_check::nll::type_check::type_check_internal
  13: rustc_mir::borrow_check::nll::compute_regions
  14: rustc_mir::borrow_check::do_mir_borrowck
  15: rustc::ty::context::GlobalCtxt::enter_local
  16: rustc_mir::borrow_check::mir_borrowck
  17: rustc::ty::query::__query_compute::mir_borrowck
  18: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::mir_borrowck<'tcx>>::compute
  19: rustc::ty::context::tls::with_context
  20: rustc::dep_graph::graph::DepGraph::with_task_impl
  21: rustc::ty::context::tls::with_related_context
  22: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  23: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  24: rustc::ty::query::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::mir_borrowck
  25: rustc::ty::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::par_body_owners
  26: rustc::util::common::time
  27: rustc::ty::context::tls::enter_context
  28: <std::thread::local::LocalKey<T>>::with
  29: rustc::ty::context::TyCtxt::create_and_enter
  30: rustc_driver::driver::compile_input
  31: rustc_driver::run_compiler_with_pool
  32: <scoped_tls::ScopedKey<T>>::set
  33: syntax::with_globals
  34: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  35: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:105
  36: rustc_driver::run
  37: rustc_driver::main
  38: std::rt::lang_start::{{closure}}
  39: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:310
  40: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:105
  41: std::rt::lang_start_internal
             at libstd/panicking.rs:289
             at libstd/panic.rs:392
             at libstd/rt.rs:58
  42: main
  43: __libc_start_main
  44: <unknown>
query stack during panic:
#0 [mir_borrowck] processing `<context::Drain<A> as actix_inner::ActorFuture>::poll`
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.28.0-nightly (f28c7aef7 2018-06-19) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z borrowck=mir -Z polonius -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `actix-web`.

To learn more, run the command again with --verbose.
