rust
error: internal compiler error: librustc/traits/specialize/mod.rs:203: failed to fully normalize <T as Trait<<T as std::iter::Iterator>::Item>>: [FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<T as std::iter::Iterator>)),depth=0),Unimplemented)]

thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:600:9
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
             at libstd/panicking.rs:480
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::bug
   8: rustc::util::bug::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::util::bug::opt_span_bug_fmt
  13: rustc::util::bug::bug_fmt
  14: rustc::ty::context::tls::with_related_context
  15: rustc::infer::InferCtxtBuilder::enter
  16: rustc::traits::specialize::specializes
  17: rustc::ty::query::__query_compute::specializes
  18: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::specializes<'tcx>>::compute
  19: rustc::dep_graph::graph::DepGraph::with_task_impl
  20: rustc::ty::context::tls::with_related_context
  21: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  22: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  23: rustc::ty::context::tls::with_related_context
  24: rustc::infer::InferCtxtBuilder::enter
  25: rustc::traits::coherence::overlapping_impls
  26: rustc::traits::specialize::specialization_graph::Graph::insert
  27: rustc::traits::specialize::specialization_graph_provider
  28: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::specialization_graph_of<'tcx>>::compute
  29: rustc::dep_graph::graph::DepGraph::with_task_impl
  30: rustc::ty::context::tls::with_related_context
  31: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  32: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  33: rustc_typeck::coherence::coherent_trait
  34: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::coherent_trait<'tcx>>::compute
  35: rustc::dep_graph::graph::DepGraph::with_task_impl
  36: rustc::ty::context::tls::with_related_context
  37: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  38: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  39: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::ensure_query
  40: rustc_typeck::coherence::check_coherence
  41: rustc::util::common::time
  42: rustc_typeck::check_crate
  43: rustc::ty::context::tls::enter_context
  44: <std::thread::local::LocalKey<T>>::with
  45: rustc::ty::context::TyCtxt::create_and_enter
  46: rustc_driver::driver::compile_input
  47: rustc_driver::run_compiler_with_pool
  48: rustc_driver::driver::spawn_thread_pool
  49: rustc_driver::run_compiler
  50: <scoped_tls::ScopedKey<T>>::set
  51: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  52: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  53: rustc_driver::run
  54: rustc_driver::main
  55: std::rt::lang_start::{{closure}}
  56: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:310
  57: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  58: std::rt::lang_start_internal
             at libstd/panicking.rs:289
             at libstd/panic.rs:392
             at libstd/rt.rs:58
  59: main
  60: __libc_start_main
  61: <unknown>
query stack during panic:
#0 [specializes] computing whether impls specialize one another
#1 [specialization_graph_of] processing `Trait`
#2 [coherent_trait] coherence checking all impls of trait `Trait`
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.31.0-nightly (f99911a4a 2018-10-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin
