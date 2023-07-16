
error: internal compiler error: src/librustc/hir/def.rs:265: attempted .def_id() on invalid def: SelfCtor(DefId(0/0:4 ~ playground[8317]::{{impl}}[0]))

thread 'main' panicked at 'Box<Any>', src/librustc_errors/lib.rs:600:9
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at src/libstd/sys_common/backtrace.rs:59
             at src/libstd/panicking.rs:211
   3: std::panicking::default_hook
             at src/libstd/panicking.rs:227
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:495
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::bug
   8: rustc::util::bug::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::util::bug::opt_span_bug_fmt
  13: rustc::util::bug::bug_fmt
  14: rustc::hir::def::Def::def_id::{{closure}}
  15: rustc::hir::def::Def::def_id
  16: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::confirm_builtin_call
  17: rustc_typeck::check::FnCtxt::check_expr_kind
  18: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs
  19: rustc_typeck::check::FnCtxt::check_block_with_expected
  20: rustc_typeck::check::FnCtxt::check_expr_kind
  21: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs
  22: rustc_typeck::check::FnCtxt::check_return_expr
  23: rustc_typeck::check::check_fn
  24: rustc::ty::context::tls::with_related_context
  25: rustc::infer::InferCtxtBuilder::enter
  26: rustc_typeck::check::typeck_tables_of
  27: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_tables_of<'tcx>>::compute
  28: rustc::dep_graph::graph::DepGraph::with_task_impl
  29: <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start
  30: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  31: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  32: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::ensure_query
  33: rustc::session::Session::track_errors
  34: rustc_typeck::check::typeck_item_bodies
  35: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_item_bodies<'tcx>>::compute
  36: rustc::dep_graph::graph::DepGraph::with_task_impl
  37: <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start
  38: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  39: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  40: rustc::util::common::time
  41: rustc_typeck::check_crate
  42: rustc::ty::context::tls::enter_context
  43: <std::thread::local::LocalKey<T>>::with
  44: rustc::ty::context::TyCtxt::create_and_enter
  45: rustc_driver::driver::compile_input
  46: rustc_driver::run_compiler_with_pool
  47: <scoped_tls::ScopedKey<T>>::set
  48: rustc_driver::run_compiler
  49: rustc_driver::monitor::{{closure}}
  50: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:102
  51: rustc_driver::run
  52: rustc_driver::main
  53: std::rt::lang_start::{{closure}}
  54: std::panicking::try::do_call
             at src/libstd/rt.rs:59
             at src/libstd/panicking.rs:310
  55: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:102
  56: std::rt::lang_start_internal
             at src/libstd/panicking.rs:289
             at src/libstd/panic.rs:398
             at src/libstd/rt.rs:58
  57: main
  58: __libc_start_main
  59: <unknown>
query stack during panic:
#0 [typeck_tables_of] processing `Foo::new`
#1 [typeck_item_bodies] type-checking all item bodies
end of query stack
error: aborting due to previous error
