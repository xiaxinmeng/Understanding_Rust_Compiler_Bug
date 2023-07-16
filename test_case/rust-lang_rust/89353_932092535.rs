
error: internal compiler error: Encountered error `Unimplemented` selecting `Binder(<<Thing<1_u32> as ThingTrait>::Type as ThingImpl>, [])` during codegen

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1181:27
stack backtrace:
   0: rust_begin_unwind
   1: core::panicking::panic_fmt
   2: rustc_errors::HandlerInner::emit_diagnostic
   3: rustc_errors::HandlerInner::emit_diag_at_span
   4: rustc_errors::HandlerInner::span_bug
   5: rustc_session::session::Session::delay_span_bug
   6: rustc_infer::infer::InferCtxtBuilder::enter
   7: rustc_trait_selection::traits::codegen::codegen_fulfill_obligation
   8: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
   9: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  10: rustc_data_structures::stack::ensure_sufficient_stack
  11: rustc_query_system::query::plumbing::get_query
  12: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::codegen_fulfill_obligation
  13: rustc_ty_utils::instance::inner_resolve_instance
  14: rustc_ty_utils::instance::resolve_instance
  15: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  16: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  17: rustc_data_structures::stack::ensure_sufficient_stack
  18: rustc_query_system::query::plumbing::get_query
  19: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::resolve_instance
  20: rustc_middle::ty::instance::Instance::resolve_opt_const_arg
  21: rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_resolve
  22: <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_const
  23: <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_mir_const
  24: <rustc_infer::infer::at::At as rustc_trait_selection::traits::query::normalize::AtExt>::normalize
  25: rustc_infer::infer::InferCtxtBuilder::enter
  26: core::ops::function::FnOnce::call_once
  27: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  28: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  29: rustc_data_structures::stack::ensure_sufficient_stack
  30: rustc_query_system::query::plumbing::get_query
  31: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::normalize_mir_const_after_erasing_regions
  32: rustc_middle::ty::normalize_erasing_regions::<impl rustc_middle::ty::context::TyCtxt>::normalize_erasing_regions
  33: rustc_const_eval::interpret::operand::<impl rustc_const_eval::interpret::eval_context::InterpCx<M>>::eval_operand
  34: rustc_const_eval::interpret::step::<impl rustc_const_eval::interpret::eval_context::InterpCx<M>>::run
  35: rustc_const_eval::const_eval::eval_queries::eval_to_allocation_raw_provider
  36: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  37: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  38: rustc_data_structures::stack::ensure_sufficient_stack
  39: rustc_query_system::query::plumbing::get_query
  40: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_allocation_raw
  41: rustc_const_eval::const_eval::eval_queries::eval_to_const_value_raw_provider
  42: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  43: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  44: rustc_data_structures::stack::ensure_sufficient_stack
  45: rustc_query_system::query::plumbing::get_query
  46: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_const_value_raw
  47: rustc_const_eval::const_eval::eval_queries::eval_to_const_value_raw_provider
  48: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  49: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  50: rustc_data_structures::stack::ensure_sufficient_stack
  51: rustc_query_system::query::plumbing::get_query
  52: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_const_value_raw
  53: rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_global_id
  54: rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_resolve
  55: <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_const
  56: <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_mir_const
  57: <rustc_infer::infer::at::At as rustc_trait_selection::traits::query::normalize::AtExt>::normalize
  58: rustc_infer::infer::InferCtxtBuilder::enter
  59: core::ops::function::FnOnce::call_once
  60: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  61: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  62: rustc_data_structures::stack::ensure_sufficient_stack
  63: rustc_query_system::query::plumbing::get_query
  64: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::normalize_mir_const_after_erasing_regions
  65: rustc_middle::ty::normalize_erasing_regions::<impl rustc_middle::ty::context::TyCtxt>::normalize_erasing_regions
  66: rustc_const_eval::interpret::operand::<impl rustc_const_eval::interpret::eval_context::InterpCx<M>>::eval_operand
  67: rustc_const_eval::interpret::step::<impl rustc_const_eval::interpret::eval_context::InterpCx<M>>::eval_rvalue_into_place
  68: <rustc_mir_transform::const_prop::ConstPropagator as rustc_middle::mir::visit::MutVisitor>::visit_statement
  69: <rustc_mir_transform::const_prop::ConstPropagator as rustc_middle::mir::visit::MutVisitor>::visit_body
  70: <rustc_mir_transform::const_prop::ConstProp as rustc_middle::mir::MirPass>::run_pass
  71: rustc_mir_transform::run_passes
  72: rustc_mir_transform::optimized_mir
  73: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  74: rustc_data_structures::stack::ensure_sufficient_stack
  75: rustc_query_system::query::plumbing::try_execute_query
  76: rustc_query_system::query::plumbing::get_query
  77: rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::instance_mir
  78: rustc_monomorphize::collector::collect_neighbours
  79: rustc_monomorphize::collector::collect_items_rec
  80: rustc_session::utils::<impl rustc_session::session::Session>::time
  81: rustc_monomorphize::collector::collect_crate_mono_items
  82: rustc_monomorphize::partitioning::collect_and_partition_mono_items
  83: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  84: rustc_data_structures::stack::ensure_sufficient_stack
  85: rustc_query_system::query::plumbing::try_execute_query
  86: rustc_query_system::query::plumbing::get_query
  87: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::collect_and_partition_mono_items
  88: rustc_codegen_ssa::base::codegen_crate
  89: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
  90: rustc_session::utils::<impl rustc_session::session::Session>::time
  91: rustc_interface::queries::Queries::ongoing_codegen

query stack during panic:
#0 [codegen_fulfill_obligation] checking if `ThingImpl` fulfills its obligations
#1 [resolve_instance] resolving instance `<<Thing<1_u32> as ThingTrait>::Type as ThingImpl>::VALUE`
#2 [normalize_mir_const_after_erasing_regions] normalizing `<<Thing<1_u32> as ThingTrait>::Type as ThingImpl>::VALUE`
#3 [eval_to_allocation_raw] const-evaluating + checking `main::promoted[0]`
#4 [eval_to_const_value_raw] simplifying constant for the type system `main::promoted[0]`
#5 [eval_to_const_value_raw] simplifying constant for the type system `main::promoted[0]`
#6 [normalize_mir_const_after_erasing_regions] normalizing `main::promoted[0]`
#7 [optimized_mir] optimizing MIR for `main`
#8 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack

