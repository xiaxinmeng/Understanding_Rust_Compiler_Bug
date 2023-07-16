

---- src/lib.rs - returns_fn_ptr (line 3) stdout ----
error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:778: Cannot create local mono-item for DefId(15:13 ~ playground[517b]::state[0])

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:644:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.34/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.34/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:47
   3: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:36
   4: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:200
   5: std::panicking::default_hook
             at src/libstd/panicking.rs:211
   6: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:477
   7: std::panicking::begin_panic
   8: rustc_errors::Handler::bug
   9: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  10: rustc::ty::context::tls::with_opt::{{closure}}
  11: rustc::ty::context::tls::with_context_opt
  12: rustc::ty::context::tls::with_opt
  13: rustc::util::bug::opt_span_bug_fmt
  14: rustc::util::bug::bug_fmt
  15: rustc_mir::monomorphize::collector::should_monomorphize_locally
  16: rustc_mir::monomorphize::collector::collect_miri
  17: <rustc_mir::monomorphize::collector::RootCollector as rustc::hir::itemlikevisit::ItemLikeVisitor>::visit_item
  18: rustc::hir::Crate::visit_all_item_likes
  19: rustc_mir::monomorphize::collector::collect_roots
  20: rustc::util::common::time
  21: rustc_mir::monomorphize::collector::collect_crate_mono_items
  22: rustc::util::common::time
  23: rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items
  24: rustc::ty::query::__query_compute::collect_and_partition_mono_items
  25: rustc::dep_graph::graph::DepGraph::with_task_impl
  26: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  27: rustc_codegen_ssa::base::codegen_crate
  28: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  29: rustc::util::common::time
  30: rustc_interface::passes::start_codegen
  31: rustc::ty::context::tls::enter_global
  32: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  33: rustc_interface::passes::create_global_ctxt::{{closure}}
  34: rustc_interface::passes::BoxedGlobalCtxt::enter
  35: rustc_interface::queries::Query<T>::compute
  36: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::compile
  37: rustc_interface::interface::run_compiler_in_existing_thread_pool
  38: std::thread::local::LocalKey<T>::with
  39: scoped_tls::ScopedKey<T>::set
  40: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
error: aborting due to previous error

