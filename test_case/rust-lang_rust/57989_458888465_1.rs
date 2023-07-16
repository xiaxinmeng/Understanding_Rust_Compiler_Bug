
error: internal compiler error: src/librustc_mir/borrow_check/places_conflict.rs:220: Tracking borrow behind shared reference.

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:605:9
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
   7: rustc_errors::Handler::bug
   8: rustc::util::bug::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::util::bug::opt_span_bug_fmt
  13: rustc::util::bug::bug_fmt
  14: rustc_mir::borrow_check::places_conflict::unroll_place
  15: rustc_mir::borrow_check::places_conflict::unroll_place
  16: rustc_mir::borrow_check::places_conflict::unroll_place
  17: rustc_mir::borrow_check::places_conflict::unroll_place
  18: rustc_mir::dataflow::impls::borrows::Borrows::kill_borrows_on_place
  19: <rustc_mir::dataflow::impls::borrows::Borrows<'a, 'gcx, 'tcx> as rustc_mir::dataflow::BitDenotation<'tcx>>::statement_effect
  20: rustc_mir::dataflow::do_dataflow
  21: rustc_mir::borrow_check::do_mir_borrowck
  22: rustc::ty::context::GlobalCtxt::enter_local
  23: rustc_mir::borrow_check::mir_borrowck
  24: rustc::ty::query::__query_compute::mir_borrowck
  25: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::mir_borrowck<'tcx>>::compute
  26: rustc::dep_graph::graph::DepGraph::with_task_impl
  27: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_get_with
  28: rustc::ty::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::par_body_owners
  29: rustc::util::common::time
  30: <std::thread::local::LocalKey<T>>::with
  31: rustc::ty::context::TyCtxt::create_and_enter
  32: rustc_driver::driver::compile_input
  33: rustc_driver::run_compiler_with_pool
  34: <scoped_tls::ScopedKey<T>>::set
  35: rustc_driver::run_compiler
  36: <scoped_tls::ScopedKey<T>>::set
query stack during panic:
#0 [mir_borrowck] processing `A::f`
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.34.0-nightly (c1c3c4e95 2019-01-29) running on x86_64-unknown-linux-gnu
