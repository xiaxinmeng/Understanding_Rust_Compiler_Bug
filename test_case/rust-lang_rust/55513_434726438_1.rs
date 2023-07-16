
thread 'main' panicked at 'expected fn type', libcore/option.rs:1008:5
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at libstd/sys_common/backtrace.rs:59
             at libstd/panicking.rs:211
   3: std::panicking::default_hook
             at libstd/panicking.rs:227
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:480
   6: std::panicking::continue_panic_fmt
             at libstd/panicking.rs:390
   7: rust_begin_unwind
             at libstd/panicking.rs:325
   8: core::panicking::panic_fmt
             at libcore/panicking.rs:77
   9: core::option::expect_failed
             at libcore/option.rs:1008
  10: rustc_typeck::check::_match::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::check_pat_tuple_struct
  11: rustc_typeck::check::_match::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::check_pat_walk
  12: rustc_typeck::check::_match::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::check_pat_tuple_struct
  13: rustc_typeck::check::_match::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::check_pat_walk
  14: rustc_typeck::check::FnCtxt::check_expr_kind
  15: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs
  16: rustc_typeck::check::FnCtxt::check_block_with_expected
  17: rustc_typeck::check::FnCtxt::check_expr_kind
  18: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs
  19: rustc_typeck::check::FnCtxt::check_return_expr
  20: rustc_typeck::check::check_fn
  21: rustc::ty::context::tls::with_related_context
  22: rustc::infer::InferCtxtBuilder::enter
  23: rustc_typeck::check::typeck_tables_of
  24: rustc::ty::query::__query_compute::typeck_tables_of
  25: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_tables_of<'tcx>>::compute
  26: rustc::dep_graph::graph::DepGraph::with_task_impl
  27: rustc::ty::context::tls::with_related_context
  28: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  29: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  30: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::ensure_query
  31: rustc::session::Session::track_errors
  32: rustc_typeck::check::typeck_item_bodies
  33: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_item_bodies<'tcx>>::compute
  34: rustc::dep_graph::graph::DepGraph::with_task_impl
  35: rustc::ty::context::tls::with_related_context
  36: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  37: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  38: rustc_typeck::check_crate
  39: rustc::ty::context::tls::enter_context
  40: <std::thread::local::LocalKey<T>>::with
  41: rustc::ty::context::TyCtxt::create_and_enter
  42: rustc_driver::driver::compile_input
  43: rustc_driver::run_compiler_with_pool
  44: rustc_driver::driver::spawn_thread_pool
  45: rustc_driver::run_compiler
  46: <scoped_tls::ScopedKey<T>>::set
  47: syntax::with_globals
  48: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  49: rustc_driver::run
  50: rustc_driver::main
  51: std::rt::lang_start::{{closure}}
  52: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:310
  53: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  54: std::rt::lang_start_internal
             at libstd/panicking.rs:289
             at libstd/panic.rs:392
             at libstd/rt.rs:58
  55: main
  56: __libc_start_main
  57: <unknown>
query stack during panic:
#0 [typeck_tables_of] processing `main`
#1 [typeck_item_bodies] type-checking all item bodies
end of query stack
