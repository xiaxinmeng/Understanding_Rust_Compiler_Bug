
error: internal compiler error: src/librustc_mir/interpret/intern.rs:182: const qualif failed to prevent mutable references

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:643:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:47
   3: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:36
   4: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:198
   5: std::panicking::default_hook
             at src/libstd/panicking.rs:212
   6: rustc::util::common::panic_hook
   7: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:479
   8: std::panicking::begin_panic
   9: rustc_errors::Handler::bug
  10: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  11: rustc::ty::context::tls::with_opt::{{closure}}
  12: rustc::ty::context::tls::with_context_opt
  13: rustc::ty::context::tls::with_opt
  14: rustc::util::bug::opt_span_bug_fmt
  15: rustc::util::bug::bug_fmt
  16: <rustc_mir::interpret::intern::InternVisitor as rustc_mir::interpret::visitor::ValueVisitor<rustc_mir::const_eval::CompileTimeInterpreter>>::visit_primitive
  17: rustc_mir::interpret::visitor::ValueVisitor::walk_value
  18: rustc_mir::interpret::intern::intern_const_alloc_recursive
  19: rustc_mir::const_eval::eval_body_using_ecx
  20: rustc_mir::const_eval::eval_promoted
  21: rustc_mir::transform::const_prop::ConstPropagator::use_ecx
  22: rustc_mir::transform::const_prop::ConstPropagator::eval_place::{{closure}}
  23: rustc::mir::Place::iterate2
  24: rustc_mir::transform::const_prop::ConstPropagator::const_prop
  25: <rustc_mir::transform::const_prop::ConstPropagator as rustc::mir::visit::MutVisitor>::visit_statement
  26: <rustc_mir::transform::const_prop::ConstProp as rustc_mir::transform::MirPass>::run_pass
  27: rustc_mir::transform::run_passes::{{closure}}
  28: rustc_mir::transform::run_passes
  29: rustc_mir::transform::optimized_mir
  30: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::optimized_mir>::compute
  31: rustc::dep_graph::graph::DepGraph::with_task_impl
  32: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  33: rustc_mir::monomorphize::collector::collect_items_rec
  34: rustc_mir::monomorphize::collector::collect_items_rec
  35: rustc_mir::monomorphize::collector::collect_items_rec
  36: rustc_mir::monomorphize::collector::collect_crate_mono_items::{{closure}}
  37: rustc::util::common::time
  38: rustc_mir::monomorphize::collector::collect_crate_mono_items
  39: rustc::util::common::time
  40: rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items
  41: rustc::ty::query::__query_compute::collect_and_partition_mono_items
  42: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::collect_and_partition_mono_items>::compute
  43: rustc::dep_graph::graph::DepGraph::with_task_impl
  44: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  45: rustc_codegen_ssa::base::codegen_crate
  46: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  47: rustc::util::common::time
  48: rustc_interface::passes::start_codegen
  49: rustc::ty::context::tls::enter_global
  50: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  51: rustc_interface::passes::create_global_ctxt::{{closure}}
  52: rustc_interface::passes::BoxedGlobalCtxt::enter
  53: rustc_interface::queries::Query<T>::compute
  54: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::ongoing_codegen
  55: rustc_interface::interface::run_compiler_in_existing_thread_pool
  56: std::thread::local::LocalKey<T>::with
  57: scoped_tls::ScopedKey<T>::set
  58: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
query stack during panic:
#0 [optimized_mir] processing `wire::tcp::test::test_tcp_options`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: aborting due to previous error
