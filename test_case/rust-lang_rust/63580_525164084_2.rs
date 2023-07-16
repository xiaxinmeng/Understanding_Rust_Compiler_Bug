
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.35/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.35/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:47
   3: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:36
   4: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:200
   5: std::panicking::default_hook
             at src/libstd/panicking.rs:214
   6: rustc::util::common::panic_hook
   7: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:481
   8: std::panicking::continue_panic_fmt
             at src/libstd/panicking.rs:384
   9: rust_begin_unwind
             at src/libstd/panicking.rs:311
  10: core::panicking::panic_fmt
             at src/libcore/panicking.rs:85
  11: core::panicking::panic_bounds_check
             at src/libcore/panicking.rs:61
  12: rustc_mir::interpret::eval_context::InterpCx<M>::load_mir
  13: rustc_mir::const_eval::const_eval_raw_provider
  14: rustc::ty::query::__query_compute::const_eval_raw
  15: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_raw>::compute
  16: rustc::dep_graph::graph::DepGraph::with_task_impl
  17: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  18: rustc_mir::const_eval::const_eval_raw_provider
  19: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_raw>::compute::{{closure}}
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc/ty/query/plumbing.rs:999
  20: rustc::ty::query::__query_compute::const_eval_raw
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc/ty/query/plumbing.rs:950
  21: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_raw>::compute
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc/ty/query/plumbing.rs:991
  22: rustc::dep_graph::graph::DepGraph::with_task_impl
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc/dep_graph/graph.rs:334
  23: rustc::dep_graph::graph::DepGraph::with_task
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc/dep_graph/graph.rs:202
  24: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}::{{closure}}
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc/ty/query/plumbing.rs:558
  25: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc/ty/query/plumbing.rs:277
  26: rustc::ty::context::tls::enter_context::{{closure}}
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc/ty/context.rs:1847
  27: rustc::ty::context::tls::set_tlv
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc/ty/context.rs:1780
  28: rustc::ty::context::tls::enter_context
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc/ty/context.rs:1846
  29: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc/ty/query/plumbing.rs:276
  30: rustc::ty::context::tls::with_related_context::{{closure}}
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc/ty/context.rs:1953
  31: rustc::ty::context::tls::with_context::{{closure}}
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc/ty/context.rs:1936
  32: rustc::ty::context::tls::with_context_opt
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc/ty/context.rs:1925
  33: rustc::ty::context::tls::with_context
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc/ty/context.rs:1936
  34: rustc::ty::context::tls::with_related_context
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc/ty/context.rs:1949
  35: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc/ty/query/plumbing.rs:265
  36: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc/ty/query/plumbing.rs:550
  37: rustc::ty::query::plumbing::with_diagnostics
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc/ty/query/plumbing.rs:210
  38: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc/ty/query/plumbing.rs:549
  39: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc/ty/query/plumbing.rs:378
  40: rustc::ty::query::TyCtxtAt::const_eval_raw
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc/ty/query/plumbing.rs:1076
  41: rustc_mir::interpret::eval_context::InterpCx<M>::const_eval_raw
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc_mir/interpret/eval_context.rs:672
  42: rustc_mir::interpret::place::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_static_to_mplace
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc_mir/interpret/place.rs:590
  43: rustc_mir::interpret::place::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_place::{{closure}}
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc_mir/interpret/place.rs:660
  44: rustc::mir::Place::iterate_over
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc/mir/mod.rs:1925
  45: rustc::mir::Place::iterate
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc/mir/mod.rs:1892
  46: rustc_mir::interpret::place::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_place
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc_mir/interpret/place.rs:635
  47: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_rvalue_into_place
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc_mir/interpret/step.rs:243
  48: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::statement
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc_mir/interpret/step.rs:85
  49: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::step
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc_mir/interpret/step.rs:61
  50: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::run
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc_mir/interpret/step.rs:40
  51: miri::eval::eval_main::{{closure}}
             at src/eval.rs:188
  52: miri::eval::eval_main
             at src/eval.rs:187
  53: <miri::MiriCompilerCalls as rustc_driver::Callbacks>::after_analysis::{{closure}}
             at src/bin/miri.rs:52
  54: rustc_interface::passes::BoxedGlobalCtxt::enter::{{closure}}::{{closure}}
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc_interface/passes.rs:817
  55: rustc::ty::context::tls::enter_global::{{closure}}
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc/ty/context.rs:1879
  56: rustc::ty::context::tls::enter_context::{{closure}}
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc/ty/context.rs:1847
  57: rustc::ty::context::tls::set_tlv
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc/ty/context.rs:1780
  58: rustc::ty::context::tls::enter_context
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc/ty/context.rs:1846
  59: rustc::ty::context::tls::enter_global
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc/ty/context.rs:1878
  60: rustc_interface::passes::BoxedGlobalCtxt::enter::{{closure}}
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc_interface/passes.rs:817
  61: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
             at ./<::rustc_data_structures::box_region::declare_box_region_type macros>:21
  62: rustc_interface::passes::create_global_ctxt::{{closure}}
  63: alloc::boxed::<impl core::ops::generator::Generator for core::pin::Pin<alloc::boxed::Box<G>>>::resume
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/liballoc/boxed.rs:1073
  64: rustc_data_structures::box_region::PinnedGenerator<I,A,R>::access
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc_data_structures/box_region.rs:52
  65: rustc_interface::passes::BoxedGlobalCtxt::access
             at ./<::rustc_data_structures::box_region::declare_box_region_type macros>:24
  66: rustc_interface::passes::BoxedGlobalCtxt::enter
             at /rustc/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/src/librustc_interface/passes.rs:817
  67: <miri::MiriCompilerCalls as rustc_driver::Callbacks>::after_analysis
             at src/bin/miri.rs:45
  68: rustc_interface::interface::run_compiler_in_existing_thread_pool
  69: std::thread::local::LocalKey<T>::with
  70: scoped_tls::ScopedKey<T>::set
  71: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
query stack during panic:
#0 [const_eval_raw] const-evaluating `main`
#1 [const_eval_raw] const-evaluating `main`
    --> /home/r/.rustup/toolchains/0444b9f66acb5da23dc816e0d8eb59623ba9ea50/lib/rustlib/src/rust/src/libstd/collections/hash/map.rs:2452:9
     |
2452 |         KEYS.with(|keys| {
     |         ^^^^
end of query stack
