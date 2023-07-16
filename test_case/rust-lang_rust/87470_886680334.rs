
thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1066:27
stack backtrace:
   0: std::panicking::begin_panic
   1: rustc_errors::HandlerInner::emit_diagnostic
   2: rustc_errors::HandlerInner::emit_diag_at_span
   3: rustc_errors::HandlerInner::span_bug
   4: rustc_errors::Handler::delay_span_bug
   5: rustc_infer::infer::InferCtxtBuilder::enter
   6: rustc_trait_selection::traits::codegen::codegen_fulfill_obligation
   7: rustc_query_system::query::plumbing::get_query_impl
   8: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::codegen_fulfill_obligation
   9: rustc_ty_utils::instance::inner_resolve_instance
  10: rustc_ty_utils::instance::resolve_instance
  11: rustc_query_system::query::plumbing::get_query_impl
  12: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::resolve_instance
  13: rustc_middle::ty::instance::Instance::resolve_opt_const_arg
  14: rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_resolve
  15: <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_const
  16: <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_mir_const
  17: <rustc_infer::infer::at::At as rustc_trait_selection::traits::query::normalize::AtExt>::normalize
  18: rustc_infer::infer::InferCtxtBuilder::enter
  19: core::ops::function::FnOnce::call_once
  20: rustc_query_system::query::plumbing::get_query_impl
  21: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::normalize_mir_const_after_erasing_regions
  22: rustc_middle::ty::normalize_erasing_regions::<impl rustc_middle::ty::context::TyCtxt>::normalize_erasing_regions
  23: rustc_mir::interpret::eval_context::InterpCx<M>::push_stack_frame
  24: rustc_mir::const_eval::eval_queries::eval_to_allocation_raw_provider
  25: rustc_query_system::query::plumbing::get_query_impl
  26: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_allocation_raw
  27: rustc_mir::const_eval::eval_queries::eval_to_const_value_raw_provider
  28: rustc_query_system::query::plumbing::get_query_impl
  29: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_const_value_raw
  30: rustc_mir::const_eval::eval_queries::eval_to_const_value_raw_provider
  31: rustc_query_system::query::plumbing::get_query_impl
  32: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_const_value_raw
  33: rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_global_id
  34: rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_resolve
  35: <&rustc_middle::ty::TyS as rustdoc::clean::Clean<rustdoc::clean::types::Type>>::clean
  36: <(rustc_span::def_id::DefId,rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::FnSig>) as rustdoc::clean::Clean<rustdoc::clean::types::FnDecl>>::clean
  37: <rustc_middle::ty::assoc::AssocItem as rustdoc::clean::Clean<rustdoc::clean::types::Item>>::clean
  38: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
  39: rustdoc::clean::inline::build_external_trait
  40: rustdoc::clean::inline::record_extern_trait
  41: rustdoc::clean::inline::build_impl
  42: rustdoc::passes::collect_trait_impls::collect_trait_impls
  43: rustdoc::core::run_global_ctxt
  44: rustc_interface::interface::create_compiler_and_run
  45: rustdoc::main_options
  46: scoped_tls::ScopedKey<T>::set
