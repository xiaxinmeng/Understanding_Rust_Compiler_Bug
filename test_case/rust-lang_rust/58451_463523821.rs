none
error: internal compiler error: src/librustc/infer/lexical_region_resolve/mod.rs:632: collect_error_for_expanding_node() could not find error for var '_#4r in universe U6, lower_bounds=[
    RegionAndOrigin(RePlaceholder(Placeholder { universe: U6, name: BrNamed(crate0:DefIndex(1:10), 'a) }),Subtype(TypeTrace(ObligationCause { span: src/main.rs:8:5: 8:6, body_id: NodeId(45), code: ItemObligation(DefId(0/0:3 ~ playground[1383]::f[0])) }))),
    RegionAndOrigin(RePlaceholder(Placeholder { universe: U6, name: BrNamed(crate0:DefIndex(1:10), 'a) }),Subtype(TypeTrace(ObligationCause { span: src/main.rs:8:5: 8:6, body_id: NodeId(45), code: ItemObligation(DefId(0/0:3 ~ playground[1383]::f[0])) })))
], upper_bounds=[
    RegionAndOrigin(RePlaceholder(Placeholder { universe: U6, name: BrNamed(crate0:DefIndex(1:10), 'a) }),Subtype(TypeTrace(ObligationCause { span: src/main.rs:8:5: 8:6, body_id: NodeId(45), code: ItemObligation(DefId(0/0:3 ~ playground[1383]::f[0])) }))),
    RegionAndOrigin(RePlaceholder(Placeholder { universe: U6, name: BrNamed(crate0:DefIndex(1:10), 'a) }),Subtype(TypeTrace(ObligationCause { span: src/main.rs:8:5: 8:6, body_id: NodeId(45), code: ItemObligation(DefId(0/0:3 ~ playground[1383]::f[0])) })))
]
 --> src/main.rs:8:5
  |
8 |     f(&[f()]);
  |     ^

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:534:9
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
  14: rustc::infer::lexical_region_resolve::resolve
  15: rustc::infer::InferCtxt::resolve_regions_and_report_errors
  16: rustc_typeck::check::regionck::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::regionck_fn
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
  37: syntax::with_globals
  38: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:90
  39: <F as alloc::boxed::FnBox<A>>::call_box
  40: std::sys::unix::thread::Thread::new::thread_start
             at /rustc/e54494727855cd14229f5d456591ed2a2f027c46/src/liballoc/boxed.rs:744
             at src/libstd/sys_common/thread.rs:14
             at src/libstd/sys/unix/thread.rs:81
  41: start_thread
  42: __clone
query stack during panic:
#0 [typeck_tables_of] processing `main`
#1 [typeck_item_bodies] type-checking all item bodies
end of query stack
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0061`.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.34.0-nightly (e54494727 2019-02-13) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin
