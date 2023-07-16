
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:578:9
stack backtrace:
   0: std::sys_common::backtrace::print
   1: std::panicking::default_hook::{{closure}}
   2: std::panicking::default_hook
   3: rustc::util::common::panic_hook
   4: std::panicking::rust_panic_with_hook
   5: std::panicking::begin_panic
   6: rustc_errors::Handler::span_bug
   7: rustc::util::bug::opt_span_bug_fmt::{{closure}}
   8: rustc::ty::context::tls::with_opt::{{closure}}
   9: rustc::ty::context::tls::with_context_opt
  10: rustc::ty::context::tls::with_opt
  11: rustc::util::bug::opt_span_bug_fmt
  12: rustc::util::bug::span_bug_fmt
  13: <rustc::ty::subst::SubstFolder as rustc::ty::fold::TypeFolder>::fold_ty
  14: <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter
  15: rustc::ty::fold::TypeFoldable::fold_with
  16: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable for &rustc::ty::TyS>::super_fold_with
  17: <rustc::ty::subst::SubstFolder as rustc::ty::fold::TypeFolder>::fold_ty
  18: <rustc::ty::subst::SubstFolder as rustc::ty::fold::TypeFolder>::fold_const
  19: rustc_mir::interpret::operand::<impl rustc_mir::interpret::eval_context::InterpretCx<M>>::eval_operand
  20: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::InterpretCx<M>>::run
  21: rustc_mir::const_eval::eval_body_using_ecx
  22: rustc_mir::const_eval::const_eval_raw_provider
  23: rustc::ty::query::__query_compute::const_eval_raw
  24: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_raw>::compute
  25: rustc::dep_graph::graph::DepGraph::with_task_impl
  26: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  27: rustc_mir::const_eval::const_eval_provider
  28: rustc::ty::query::__query_compute::const_eval
  29: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval>::compute
  30: rustc::dep_graph::graph::DepGraph::with_task_impl
  31: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  32: rustc_mir::const_eval::const_eval_provider
  33: rustc::ty::query::__query_compute::const_eval
  34: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval>::compute
  35: rustc::dep_graph::graph::DepGraph::with_task_impl
  36: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  37: rustc_mir::monomorphize::collector::collect_const
  38: rustc::mir::visit::Visitor::super_rvalue
  39: <rustc_mir::monomorphize::collector::MirNeighborCollector as rustc::mir::visit::Visitor>::visit_rvalue
  40: rustc::mir::visit::Visitor::visit_body
  41: rustc_mir::monomorphize::collector::collect_items_rec
  42: rustc_mir::monomorphize::collector::collect_items_rec
  43: rustc_mir::monomorphize::collector::collect_crate_mono_items::{{closure}}
  44: rustc::util::common::time
  45: rustc_mir::monomorphize::collector::collect_crate_mono_items
  46: rustc::util::common::time
  47: rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items
  48: rustc::ty::query::__query_compute::collect_and_partition_mono_items
  49: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::collect_and_partition_mono_items>::compute
  50: rustc::dep_graph::graph::DepGraph::with_task_impl
  51: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  52: rustc_codegen_ssa::base::codegen_crate
  53: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  54: rustc::util::common::time
  55: rustc_interface::passes::start_codegen
  56: rustc::ty::context::tls::enter_global
  57: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  58: rustc_interface::passes::create_global_ctxt::{{closure}}
  59: rustc_interface::passes::BoxedGlobalCtxt::enter
  60: rustc_interface::queries::Query<T>::compute
  61: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::ongoing_codegen
  62: rustc_interface::interface::run_compiler_in_existing_thread_pool
  63: std::thread::local::LocalKey<T>::with
  64: scoped_tls::ScopedKey<T>::set
  65: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
query stack during panic:
#0 [const_eval_raw] const-evaluating `<f32 as float::Float>::EXPONENT_MASK`
#1 [const_eval] const-evaluating + checking `<f32 as float::Float>::EXPONENT_MASK`
#2 [const_eval] const-evaluating + checking `<f32 as float::Float>::EXPONENT_MASK`
#3 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: aborting due to previous error
