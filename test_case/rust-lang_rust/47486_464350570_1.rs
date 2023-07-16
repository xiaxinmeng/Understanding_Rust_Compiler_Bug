
Compiling playground v0.0.1 (/playground)
error[E0308]: mismatched types
 --> src/main.rs:2:10
  |
2 |     () < std::mem::size_of::<_>();
  |          ^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found usize
  |
  = note: expected type `()`
             found type `usize`

error: internal compiler error: src/librustc/ty/sty.rs:1837: Ty::fn_sig() called on non-fn type: [type error]

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
  14: rustc::ty::sty::<impl rustc::ty::TyS<'tcx>>::fn_sig
  15: <rustc_mir::transform::check_unsafety::UnsafetyChecker<'a, 'tcx> as rustc::mir::visit::Visitor<'tcx>>::visit_terminator
  16: rustc_mir::transform::check_unsafety::unsafety_check_result
  17: rustc::ty::query::__query_compute::unsafety_check_result
  18: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::unsafety_check_result<'tcx>>::compute
  19: rustc::dep_graph::graph::DepGraph::with_task_impl
  20: <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start
  21: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  22: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  23: rustc::ty::query::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::unsafety_check_result
  24: rustc_mir::transform::mir_const
  25: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::mir_const<'tcx>>::compute
  26: rustc::dep_graph::graph::DepGraph::with_task_impl
  27: <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start
  28: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  29: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  30: rustc_mir::transform::qualify_consts::mir_const_qualif
  31: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::mir_const_qualif<'tcx>>::compute
  32: rustc::dep_graph::graph::DepGraph::with_task_impl
  33: <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start
  34: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  35: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  36: rustc_mir::const_eval::const_eval_raw_provider
  37: rustc::ty::query::__query_compute::const_eval_raw
  38: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::const_eval_raw<'tcx>>::compute
  39: rustc::dep_graph::graph::DepGraph::with_task_impl
  40: <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start
  41: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  42: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  43: rustc::ty::query::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::const_eval_raw
  44: rustc_mir::const_eval::const_eval_provider
  45: rustc::ty::query::__query_compute::const_eval
  46: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::const_eval<'tcx>>::compute
  47: rustc::dep_graph::graph::DepGraph::with_task_impl
  48: <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start
  49: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  50: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  51: rustc::ty::query::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::const_eval
  52: rustc_typeck::check::FnCtxt::check_expr_kind
  53: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs
  54: rustc_typeck::check::FnCtxt::check_block_with_expected
  55: rustc_typeck::check::FnCtxt::check_expr_kind
  56: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs
  57: rustc_typeck::check::FnCtxt::check_return_expr
  58: rustc_typeck::check::check_fn
  59: rustc::ty::context::tls::with_related_context
  60: rustc::infer::InferCtxtBuilder::enter
  61: rustc_typeck::check::typeck_tables_of
  62: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_tables_of<'tcx>>::compute
  63: rustc::dep_graph::graph::DepGraph::with_task_impl
  64: <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start
  65: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  66: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  67: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::ensure_query
  68: rustc::session::Session::track_errors
  69: rustc_typeck::check::typeck_item_bodies
  70: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_item_bodies<'tcx>>::compute
  71: rustc::dep_graph::graph::DepGraph::with_task_impl
  72: <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start
  73: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  74: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  75: rustc::util::common::time
  76: rustc_typeck::check_crate
  77: rustc::ty::context::tls::enter_context
  78: <std::thread::local::LocalKey<T>>::with
  79: rustc::ty::context::TyCtxt::create_and_enter
  80: rustc_driver::driver::compile_input
  81: rustc_driver::run_compiler_with_pool
  82: <scoped_tls::ScopedKey<T>>::set
  83: rustc_driver::run_compiler
  84: rustc_driver::monitor::{{closure}}
  85: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:102
  86: rustc_driver::run
  87: rustc_driver::main
  88: std::rt::lang_start::{{closure}}
  89: std::panicking::try::do_call
             at src/libstd/rt.rs:59
             at src/libstd/panicking.rs:310
  90: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:102
  91: std::rt::lang_start_internal
             at src/libstd/panicking.rs:289
             at src/libstd/panic.rs:398
             at src/libstd/rt.rs:58
  92: main
  93: __libc_start_main
  94: <unknown>
query stack during panic:
#0 [unsafety_check_result] processing `main::{{constant}}`
#1 [mir_const] processing `main::{{constant}}`
#2 [mir_const_qualif] processing `main::{{constant}}`
#3 [const_eval_raw] const-evaluating `main::{{constant}}`
#4 [const_eval] const-evaluating + checking `main::{{constant}}`
#5 [typeck_tables_of] processing `main`
#6 [typeck_item_bodies] type-checking all item bodies
end of query stack
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.32.0 (9fda7c223 2019-01-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `playground`.

To learn more, run the command again with --verbose.
