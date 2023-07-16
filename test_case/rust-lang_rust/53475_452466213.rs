
thread 'rustc' panicked at 'region_obligations not empty: [
    (
        NodeId(32),
        RegionObligation(sub_region=ReStatic, sup_type=T)
    )
]', src/librustc/infer/mod.rs:1110:9
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
   6: std::panicking::continue_panic_fmt
             at src/libstd/panicking.rs:385
   7: std::panicking::begin_panic_fmt
             at src/libstd/panicking.rs:340
   8: rustc::infer::InferCtxt::resolve_regions_and_report_errors
   9: rustc::ty::context::tls::with_context::{{closure}}
  10: rustc::ty::context::GlobalCtxt::enter_local
  11: rustc_typeck::coherence::builtin::coerce_unsized_info
  12: rustc::ty::query::__query_compute::coerce_unsized_info
  13: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::coerce_unsized_info<'tcx>>::compute
  14: rustc::dep_graph::graph::DepGraph::with_task_impl
  15: <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start
  16: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  17: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_get_with
  18: rustc_typeck::coherence::builtin::check_trait
  19: rustc_typeck::coherence::coherent_trait
  20: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::coherent_trait<'tcx>>::compute
  21: rustc::dep_graph::graph::DepGraph::with_task_impl
  22: <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start
  23: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  24: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_get_with
  25: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::ensure_query
  26: rustc_typeck::coherence::check_coherence
  27: rustc::session::Session::track_errors
  28: rustc_typeck::check_crate
  29: <std::thread::local::LocalKey<T>>::with
  30: rustc::ty::context::TyCtxt::create_and_enter
  31: rustc_driver::driver::compile_input
  32: rustc_driver::run_compiler_with_pool
  33: <scoped_tls::ScopedKey<T>>::set
  34: rustc_driver::run_compiler
  35: <scoped_tls::ScopedKey<T>>::set
  36: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  37: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:92
  38: <F as alloc::boxed::FnBox<A>>::call_box
  39: std::sys::unix::thread::Thread::new::thread_start
             at /rustc/8e2063d02062ee9f088274690a97826333847e17/src/liballoc/boxed.rs:744
             at src/libstd/sys_common/thread.rs:14
             at src/libstd/sys/unix/thread.rs:81
  40: start_thread
  41: __clone
query stack during panic:
#0 [coerce_unsized_info] processing `<Foo<T> as std::ops::CoerceUnsized<Foo<(dyn std::any::Any + 'static)>>>`
  --> src/lib.rs:10:1
   |
10 | impl<T> CoerceUnsized<Foo<dyn Any>> for Foo<T> {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
#1 [coherent_trait] coherence checking all impls of trait `std::ops::CoerceUnsized`
end of query stack

error: internal compiler error: unexpected panic
