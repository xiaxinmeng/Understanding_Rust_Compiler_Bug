
error: internal compiler error: src/librustc_typeck/check/wfcheck.rs:255: inference variables in Matrix<f32, U3, U1, _>
 --> src/main.rs:6:1
  |
6 | struct Foo(Vector3<f32>);
  | ^^^^^^^^^^^^^^^^^^^^^^^^^

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:543:9
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
   7: rustc_errors::Handler::span_bug
   8: rustc::util::bug::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::util::bug::opt_span_bug_fmt
  13: rustc::util::bug::span_bug_fmt
  14: rustc_typeck::check::wfcheck::check_type_defn::{{closure}}::{{closure}}::{{closure}}
  15: rustc::ty::context::GlobalCtxt::enter_local
  16: rustc_typeck::check::wfcheck::check_item_well_formed
  17: rustc::ty::query::__query_compute::check_item_well_formed
  18: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::check_item_well_formed<'tcx>>::compute
  19: rustc::dep_graph::graph::DepGraph::with_task_impl
  20: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_get_with
  21: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::ensure_query
  22: rustc::hir::Crate::visit_all_item_likes
  23: rustc::util::common::time
  24: rustc_typeck::check_crate
  25: <std::thread::local::LocalKey<T>>::with
  26: rustc::ty::context::TyCtxt::create_and_enter
  27: rustc_driver::driver::compile_input
  28: rustc_driver::run_compiler_with_pool
  29: <scoped_tls::ScopedKey<T>>::set
  30: rustc_driver::run_compiler
  31: <scoped_tls::ScopedKey<T>>::set
query stack during panic:
#0 [check_item_well_formed] processing `Foo`
end of query stack
error: aborting due to previous error

note: rustc 1.34.0-nightly (f6fac4225 2019-02-03) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin
