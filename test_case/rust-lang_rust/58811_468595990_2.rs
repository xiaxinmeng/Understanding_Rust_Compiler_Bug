none
error: internal compiler error: src/librustc_typeck/collect.rs:1320: unexpected const parent in type_of_def_id(): Ty(type(Vec<&str, >))

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:620:9
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
  14: rustc_typeck::collect::type_of
  15: rustc::ty::query::__query_compute::type_of
  16: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::type_of<'tcx>>::compute
  17: rustc::dep_graph::graph::DepGraph::with_task_impl
  18: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  19: rustc::ty::context::GlobalCtxt::enter_local
  20: rustc_typeck::check::typeck_tables_of
  21: rustc::ty::query::__query_compute::typeck_tables_of
  22: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_tables_of<'tcx>>::compute
  23: rustc::dep_graph::graph::DepGraph::with_task_impl
  24: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  25: rustc::ty::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::par_body_owners
  26: rustc_typeck::check::typeck_item_bodies
  27: rustc::ty::query::__query_compute::typeck_item_bodies
  28: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_item_bodies<'tcx>>::compute
  29: rustc::dep_graph::graph::DepGraph::with_task_impl
  30: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  31: rustc::util::common::time
  32: rustc_typeck::check_crate
  33: <std::thread::local::LocalKey<T>>::with
  34: rustc::ty::context::TyCtxt::create_and_enter
  35: rustc_driver::driver::compile_input
  36: rustc_driver::run_compiler_with_pool
  37: <scoped_tls::ScopedKey<T>>::set
  38: rustc_driver::run_compiler
  39: syntax::with_globals
  40: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:87
  41: <F as alloc::boxed::FnBox<A>>::call_box
  42: std::sys::unix::thread::Thread::new::thread_start
             at /rustc/f66e4697ae286985ddefc53c3a047614568458bb/src/liballoc/boxed.rs:749
             at src/libstd/sys_common/thread.rs:14
             at src/libstd/sys/unix/thread.rs:81
  43: start_thread
  44: __clone
query stack during panic:
#0 [type_of] processing `main::{{constant}}`
#1 [typeck_tables_of] processing `main::{{constant}}`
#2 [typeck_item_bodies] type-checking all item bodies
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.34.0-nightly (f66e4697a 2019-02-20) running on x86_64-unknown-linux-gnu
