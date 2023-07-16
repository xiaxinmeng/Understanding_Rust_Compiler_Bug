
An error occurred in miri:
stack backtrace:
   0: <rustc::mir::interpret::error::InterpErrorInfo as core::convert::From<rustc::mir::interpret::error::InterpError>>::from
             at src/librustc/mir/interpret/error.rs:248
   1: <T as core::convert::Into<U>>::into
             at src/libcore/convert/mod.rs:558
      rustc_mir::interpret::memory::Memory<M>::get_fn_alloc
             at src/librustc_mir/interpret/memory.rs:612
   2: rustc_mir::interpret::memory::Memory<M>::get_size_and_align
             at src/librustc_mir/interpret/memory.rs:563
   3: rustc_mir::interpret::memory::Memory<M>::check_ptr_access_align
             at src/librustc_mir/interpret/memory.rs:372
   4: rustc_mir::interpret::memory::Memory<M>::check_ptr_access
             at src/librustc_mir/interpret/memory.rs:321
      rustc_mir::interpret::place::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::check_mplace_access
             at src/librustc_mir/interpret/place.rs:359
      rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::try_read_immediate_from_mplace
             at src/librustc_mir/interpret/operand.rs:263
      rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::try_read_immediate
             at src/librustc_mir/interpret/operand.rs:316
   5: rustc_mir::interpret::place::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::copy_op_no_validate
             at src/librustc_mir/interpret/place.rs:863
   6: rustc_mir::interpret::place::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::copy_op
             at src/librustc_mir/interpret/place.rs:834
      rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::eval_rvalue_into_place
             at src/librustc_mir/interpret/step.rs:150
   7: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::statement
             at src/librustc_mir/interpret/step.rs:91
      rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::step
             at src/librustc_mir/interpret/step.rs:67
      rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpCx<M>>::run
             at src/librustc_mir/interpret/step.rs:34
   8: rustc_mir::const_eval::eval_queries::eval_body_using_ecx
             at src/librustc_mir/const_eval/eval_queries.rs:56
      rustc_mir::const_eval::eval_queries::const_eval_raw_provider::{{closure}}
             at src/librustc_mir/const_eval/eval_queries.rs:306
      core::result::Result<T,E>::and_then
             at src/libcore/result.rs:727
      rustc_mir::const_eval::eval_queries::const_eval_raw_provider
             at src/librustc_mir/const_eval/eval_queries.rs:306
   9: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_raw>::compute::{{closure}}
             at src/librustc/ty/query/plumbing.rs:951
      rustc::ty::query::__query_compute::const_eval_raw
             at src/librustc/ty/query/plumbing.rs:902
  10: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_raw>::compute
             at src/librustc/ty/query/plumbing.rs:943
  11: rustc::dep_graph::graph::DepGraph::with_task_impl
             at src/librustc/dep_graph/graph.rs:341
      rustc::dep_graph::graph::DepGraph::with_task
             at src/librustc/dep_graph/graph.rs:209
  12: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}::{{closure}}
             at src/librustc/ty/query/plumbing.rs:557
      rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at src/librustc/ty/query/plumbing.rs:278
      rustc::ty::context::tls::enter_context::{{closure}}
             at src/librustc/ty/context.rs:1697
      rustc::ty::context::tls::set_tlv
             at src/librustc/ty/context.rs:1682
      rustc::ty::context::tls::enter_context
             at src/librustc/ty/context.rs:1697
      rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}
             at src/librustc/ty/query/plumbing.rs:278
      rustc::ty::context::tls::with_related_context::{{closure}}
             at src/librustc/ty/context.rs:1785
      rustc::ty::context::tls::with_context::{{closure}}
             at src/librustc/ty/context.rs:1769
      rustc::ty::context::tls::with_context_opt
             at src/librustc/ty/context.rs:1758
      rustc::ty::context::tls::with_context
             at src/librustc/ty/context.rs:1769
      rustc::ty::context::tls::with_related_context
             at src/librustc/ty/context.rs:1782
      rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query
             at src/librustc/ty/query/plumbing.rs:267
      rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}
             at src/librustc/ty/query/plumbing.rs:547
      rustc::ty::query::plumbing::with_diagnostics
             at src/librustc/ty/query/plumbing.rs:212
      rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job
             at src/librustc/ty/query/plumbing.rs:546
      rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
             at src/librustc/ty/query/plumbing.rs:380
  13: rustc::ty::query::TyCtxtAt::const_eval_raw
             at src/librustc/ty/query/plumbing.rs:1057
      rustc::ty::query::<impl rustc::ty::context::TyCtxt>::const_eval_raw
             at src/librustc/ty/query/plumbing.rs:1020
      rustc_mir::const_eval::eval_queries::const_eval_validated_provider
             at src/librustc_mir/const_eval/eval_queries.rs:253
  14: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_validated>::compute::{{closure}}
             at src/librustc/ty/query/plumbing.rs:951
      rustc::ty::query::__query_compute::const_eval_validated
             at src/librustc/ty/query/plumbing.rs:902
  15: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_validated>::compute
             at src/librustc/ty/query/plumbing.rs:943
  16: rustc::dep_graph::graph::DepGraph::with_task_impl
             at src/librustc/dep_graph/graph.rs:341
      rustc::dep_graph::graph::DepGraph::with_task
             at src/librustc/dep_graph/graph.rs:209
  17: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}::{{closure}}
             at src/librustc/ty/query/plumbing.rs:557
      rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at src/librustc/ty/query/plumbing.rs:278
      rustc::ty::context::tls::enter_context::{{closure}}
             at src/librustc/ty/context.rs:1697
      rustc::ty::context::tls::set_tlv
             at src/librustc/ty/context.rs:1682
      rustc::ty::context::tls::enter_context
             at src/librustc/ty/context.rs:1697
      rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}
             at src/librustc/ty/query/plumbing.rs:278
      rustc::ty::context::tls::with_related_context::{{closure}}
             at src/librustc/ty/context.rs:1785
      rustc::ty::context::tls::with_context::{{closure}}
             at src/librustc/ty/context.rs:1769
      rustc::ty::context::tls::with_context_opt
             at src/librustc/ty/context.rs:1758
      rustc::ty::context::tls::with_context
             at src/librustc/ty/context.rs:1769
      rustc::ty::context::tls::with_related_context
             at src/librustc/ty/context.rs:1782
      rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query
             at src/librustc/ty/query/plumbing.rs:267
      rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}
             at src/librustc/ty/query/plumbing.rs:547
      rustc::ty::query::plumbing::with_diagnostics
             at src/librustc/ty/query/plumbing.rs:212
      rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job
             at src/librustc/ty/query/plumbing.rs:546
      rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
             at src/librustc/ty/query/plumbing.rs:380
  18: rustc::ty::query::TyCtxtAt::const_eval_validated
             at src/librustc/ty/query/plumbing.rs:1057
      rustc::ty::query::<impl rustc::ty::context::TyCtxt>::const_eval_validated
             at src/librustc/ty/query/plumbing.rs:1020
      rustc_mir::const_eval::eval_queries::const_eval_validated_provider
             at src/librustc_mir/const_eval/eval_queries.rs:230
  19: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_validated>::compute::{{closure}}
             at src/librustc/ty/query/plumbing.rs:951
      rustc::ty::query::__query_compute::const_eval_validated
             at src/librustc/ty/query/plumbing.rs:902
  20: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_validated>::compute
             at src/librustc/ty/query/plumbing.rs:943
  21: rustc::dep_graph::graph::DepGraph::with_task_impl
             at src/librustc/dep_graph/graph.rs:341
      rustc::dep_graph::graph::DepGraph::with_task
             at src/librustc/dep_graph/graph.rs:209
  22: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}::{{closure}}
             at src/librustc/ty/query/plumbing.rs:557
      rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at src/librustc/ty/query/plumbing.rs:278
      rustc::ty::context::tls::enter_context::{{closure}}
             at src/librustc/ty/context.rs:1697
      rustc::ty::context::tls::set_tlv
             at src/librustc/ty/context.rs:1682
      rustc::ty::context::tls::enter_context
             at src/librustc/ty/context.rs:1697
      rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}
             at src/librustc/ty/query/plumbing.rs:278
      rustc::ty::context::tls::with_related_context::{{closure}}
             at src/librustc/ty/context.rs:1785
      rustc::ty::context::tls::with_context::{{closure}}
             at src/librustc/ty/context.rs:1769
      rustc::ty::context::tls::with_context_opt
             at src/librustc/ty/context.rs:1758
      rustc::ty::context::tls::with_context
             at src/librustc/ty/context.rs:1769
      rustc::ty::context::tls::with_related_context
             at src/librustc/ty/context.rs:1782
      rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query
             at src/librustc/ty/query/plumbing.rs:267
      rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}
             at src/librustc/ty/query/plumbing.rs:547
      rustc::ty::query::plumbing::with_diagnostics
             at src/librustc/ty/query/plumbing.rs:212
      rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job
             at src/librustc/ty/query/plumbing.rs:546
      rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
             at src/librustc/ty/query/plumbing.rs:380
  23: rustc::ty::query::TyCtxtAt::const_eval_validated
             at src/librustc/ty/query/plumbing.rs:1057
      rustc::ty::query::<impl rustc::ty::context::TyCtxt>::const_eval_validated
             at src/librustc/ty/query/plumbing.rs:1020
      rustc::mir::interpret::queries::<impl rustc::ty::context::TyCtxt>::const_eval_poly
             at src/librustc/mir/interpret/queries.rs:22
  24: <rustc_lint::builtin::UnusedBrokenConst as rustc_lint::passes::LateLintPass>::check_item
             at src/librustc_lint/builtin.rs:0
  25: <rustc_lint::BuiltinCombinedLateLintPass as rustc_lint::passes::LateLintPass>::check_item
             at src/librustc_lint/passes.rs:113
  26: <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_item::{{closure}}::{{closure}}
             at src/librustc_lint/late.rs:42
      rustc_lint::late::LateContextAndPass<T>::with_param_env
             at src/librustc_lint/late.rs:73
      <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_item::{{closure}}
             at src/librustc_lint/late.rs:131
      rustc_lint::late::LateContextAndPass<T>::with_lint_attrs
             at src/librustc_lint/late.rs:61
      <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_item
             at src/librustc_lint/late.rs:130
      rustc_hir::intravisit::Visitor::visit_nested_item
             at <::syntax::visit::walk_list macros>:2
  27: rustc_hir::intravisit::walk_mod
             at src/librustc_hir/intravisit.rs:455
  28: rustc_lint::late::LateContextAndPass<T>::process_mod
             at src/librustc_lint/late.rs:79
      <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_mod
             at src/librustc_lint/late.rs:236
  29: rustc_hir::intravisit::walk_crate
             at src/librustc_hir/intravisit.rs:441
  30: rustc_lint::late::late_lint_pass_crate::{{closure}}
             at src/librustc_lint/late.rs:427
      rustc_lint::late::LateContextAndPass<T>::with_lint_attrs
             at src/librustc_lint/late.rs:61
      rustc_lint::late::late_lint_pass_crate
             at src/librustc_lint/late.rs:422
      rustc_lint::late::late_lint_crate
             at src/librustc_lint/late.rs:441
  31: rustc_lint::late::check_crate::{{closure}}::{{closure}}
             at src/librustc_lint/late.rs:471
      rustc_data_structures::profiling::VerboseTimingGuard::run
             at src/librustc_data_structures/profiling.rs:555
      rustc_session::utils::<impl rustc_session::session::Session>::time
             at src/librustc_session/utils.rs:9
  32: rustc_lint::late::check_crate::{{closure}}
             at src/librustc_lint/late.rs:469
      rustc_data_structures::sync::join
             at src/librustc_data_structures/sync.rs:160
  33: rustc_lint::late::check_crate
             at src/librustc_lint/late.rs:467
      rustc_interface::passes::analysis::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at src/librustc_interface/passes.rs:866
      rustc_data_structures::profiling::VerboseTimingGuard::run
             at src/librustc_data_structures/profiling.rs:555
      rustc_session::utils::<impl rustc_session::session::Session>::time
             at src/librustc_session/utils.rs:9
  34: rustc_interface::passes::analysis::{{closure}}::{{closure}}::{{closure}}
             at src/librustc_interface/passes.rs:865
      core::ops::function::FnOnce::call_once
             at src/libcore/ops/function.rs:232
      <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at src/libstd/panic.rs:318
      std::panicking::try::do_call
             at src/libstd/panicking.rs:303
  35: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:86
  36: std::panicking::try
             at src/libstd/panicking.rs:281
  37: std::panic::catch_unwind
             at src/libstd/panic.rs:394
      rustc_interface::passes::analysis::{{closure}}::{{closure}}
             at src/librustc_interface/passes.rs:852
      core::ops::function::FnOnce::call_once
             at src/libcore/ops/function.rs:232
      <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at src/libstd/panic.rs:318
  38: std::panicking::try::do_call
             at src/libstd/panicking.rs:303
  39: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:86
  40: std::panicking::try
             at src/libstd/panicking.rs:281
  41: std::panic::catch_unwind
             at src/libstd/panic.rs:394
      rustc_interface::passes::analysis::{{closure}}
             at src/librustc_interface/passes.rs:848
      rustc_data_structures::profiling::VerboseTimingGuard::run
             at src/librustc_data_structures/profiling.rs:555
      rustc_session::utils::<impl rustc_session::session::Session>::time
             at src/librustc_session/utils.rs:9
  42: rustc_interface::passes::analysis
             at src/librustc_interface/passes.rs:847
  43: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::analysis>::compute::{{closure}}
             at src/librustc/ty/query/plumbing.rs:951
      rustc::ty::query::__query_compute::analysis
             at src/librustc/ty/query/plumbing.rs:902
  44: rustc::dep_graph::graph::DepGraph::with_task_impl
             at src/librustc/dep_graph/graph.rs:341
      rustc::dep_graph::graph::DepGraph::with_eval_always_task
             at src/librustc/dep_graph/graph.rs:388
  45: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}::{{closure}}
             at src/librustc/ty/query/plumbing.rs:549
      rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}
             at src/librustc/ty/query/plumbing.rs:278
      rustc::ty::context::tls::enter_context::{{closure}}
             at src/librustc/ty/context.rs:1697
      rustc::ty::context::tls::set_tlv
             at src/librustc/ty/context.rs:1682
      rustc::ty::context::tls::enter_context
             at src/librustc/ty/context.rs:1697
      rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query::{{closure}}
             at src/librustc/ty/query/plumbing.rs:278
      rustc::ty::context::tls::with_related_context::{{closure}}
             at src/librustc/ty/context.rs:1785
      rustc::ty::context::tls::with_context::{{closure}}
             at src/librustc/ty/context.rs:1769
      rustc::ty::context::tls::with_context_opt
             at src/librustc/ty/context.rs:1758
      rustc::ty::context::tls::with_context
             at src/librustc/ty/context.rs:1769
      rustc::ty::context::tls::with_related_context
             at src/librustc/ty/context.rs:1782
      rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::start_query
             at src/librustc/ty/query/plumbing.rs:267
      rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job::{{closure}}
             at src/librustc/ty/query/plumbing.rs:547
      rustc::ty::query::plumbing::with_diagnostics
             at src/librustc/ty/query/plumbing.rs:212
      rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query_with_job
             at src/librustc/ty/query/plumbing.rs:546
      rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
             at src/librustc/ty/query/plumbing.rs:380
  46: rustc::ty::query::TyCtxtAt::analysis
             at src/librustc/ty/query/plumbing.rs:1057
      rustc::ty::query::<impl rustc::ty::context::TyCtxt>::analysis
             at src/librustc/ty/query/plumbing.rs:1020
      rustc_driver::run_compiler::{{closure}}::{{closure}}::{{closure}}
             at src/librustc_driver/lib.rs:390
      rustc_interface::passes::QueryContext::enter::{{closure}}
             at src/librustc_interface/passes.rs:696
      rustc::ty::context::tls::enter_global::{{closure}}
             at src/librustc/ty/context.rs:1720
      rustc::ty::context::tls::enter_context::{{closure}}
             at src/librustc/ty/context.rs:1697
      rustc::ty::context::tls::set_tlv
             at src/librustc/ty/context.rs:1682
      rustc::ty::context::tls::enter_context
             at src/librustc/ty/context.rs:1697
      rustc::ty::context::tls::enter_global
             at src/librustc/ty/context.rs:1720
  47: rustc_interface::passes::QueryContext::enter
             at src/librustc_interface/passes.rs:696
      rustc_driver::run_compiler::{{closure}}::{{closure}}
             at src/librustc_driver/lib.rs:390
      rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
             at src/librustc_interface/queries.rs:339
      rustc_driver::run_compiler::{{closure}}
             at src/librustc_driver/lib.rs:290
      rustc_interface::interface::run_compiler_in_existing_thread_pool
             at src/librustc_interface/interface.rs:186
  48: rustc_interface::interface::run_compiler::{{closure}}
             at src/librustc_interface/interface.rs:200
      rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}
             at src/librustc_interface/util.rs:155
      scoped_tls::ScopedKey<T>::set
             at /home/wesley/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
      rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}
             at src/librustc_interface/util.rs:151
      scoped_tls::ScopedKey<T>::set
             at /home/wesley/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
      syntax::attr::with_globals::{{closure}}
             at src/libsyntax/attr/mod.rs:44
      scoped_tls::ScopedKey<T>::set
             at /home/wesley/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
  49: syntax::attr::with_globals
             at src/libsyntax/attr/mod.rs:44
  50: rustc_interface::util::spawn_thread_pool::{{closure}}
             at src/librustc_interface/util.rs:150
      rustc_interface::util::scoped_thread::{{closure}}
             at src/librustc_interface/util.rs:125
      std::sys_common::backtrace::__rust_begin_short_backtrace
             at src/libstd/sys_common/backtrace.rs:129
  51: std::thread::Builder::spawn_unchecked::{{closure}}::{{closure}}
             at src/libstd/thread/mod.rs:475
      <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at src/libstd/panic.rs:318
      std::panicking::try::do_call
             at src/libstd/panicking.rs:303
  52: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:86
  53: std::panicking::try
             at src/libstd/panicking.rs:281
  54: std::panic::catch_unwind
             at src/libstd/panic.rs:394
      std::thread::Builder::spawn_unchecked::{{closure}}
             at src/libstd/thread/mod.rs:474
      core::ops::function::FnOnce::call_once{{vtable.shim}}
             at src/libcore/ops/function.rs:232
  55: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once
             at src/liballoc/boxed.rs:1016
  56: <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once
             at src/liballoc/boxed.rs:1016
  57: std::sys_common::thread::start_thread
             at src/libstd/sys_common/thread.rs:13
  58: std::sys::unix::thread::Thread::new::thread_start
             at src/libstd/sys/unix/thread.rs:80
  59: start_thread
             at /usr/src/debug/glibc-2.30-1.2.x86_64/nptl/pthread_create.c:479
  60: __clone
