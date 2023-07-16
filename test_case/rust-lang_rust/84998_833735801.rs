
stack backtrace:
   0: rust_begin_unwind
             at /rustc/4de75720970a223b125a811d3662fd15a08d4d18/library/std/src/panicking.rs:493:5
   1: core::panicking::panic_fmt
             at /rustc/4de75720970a223b125a811d3662fd15a08d4d18/library/core/src/panicking.rs:92:14
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed
   4: rustc_query_system::query::plumbing::incremental_verify_ich
   5: rustc_query_system::query::plumbing::load_from_disk_and_cache_in_memory
   6: rustc_data_structures::stack::ensure_sufficient_stack
   7: rustc_query_system::query::plumbing::get_query_impl
   8: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::evaluate_obligation
   9: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation
  10: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation_no_overflow
  11: rustc_trait_selection::traits::type_known_to_meet_bound_modulo_regions
  12: rustc_infer::infer::InferCtxtBuilder::enter
  13: rustc_ty_utils::common_traits::is_unpin_raw
  14: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  15: rustc_query_system::query::plumbing::load_from_disk_and_cache_in_memory
  16: rustc_data_structures::stack::ensure_sufficient_stack
  17: rustc_query_system::query::plumbing::get_query_impl
  18: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::is_unpin_raw
  19: rustc_middle::ty::util::<impl rustc_middle::ty::TyS>::is_unpin
  20: rustc_middle::ty::layout::<impl rustc_target::abi::TyAndLayoutMethods<C> for &rustc_middle::ty::TyS>::pointee_info_at
  21: <rustc_target::abi::call::FnAbi<&rustc_middle::ty::TyS> as rustc_middle::ty::layout::FnAbiExt<C>>::new_internal::{{closure}}::{{closure}}
  22: rustc_target::abi::call::ArgAbi<Ty>::new
  23: core::ops::function::impls::<impl core::ops::function::FnMut<A> for &mut F>::call_mut
  24: <core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::fold
  25: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
  26: <rustc_target::abi::call::FnAbi<&rustc_middle::ty::TyS> as rustc_middle::ty::layout::FnAbiExt<C>>::new_internal
  27: <rustc_target::abi::call::FnAbi<&rustc_middle::ty::TyS> as rustc_middle::ty::layout::FnAbiExt<C>>::of_instance
  28: rustc_codegen_llvm::mono_item::<impl rustc_codegen_ssa::traits::declare::PreDefineMethods for rustc_codegen_llvm::context::CodegenCx>::predefine_fn
  29: rustc_codegen_llvm::base::compile_codegen_unit::module_codegen
  30: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  31: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  32: rustc_codegen_llvm::base::compile_codegen_unit
  33: rustc_codegen_ssa::base::codegen_crate
  34: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
  35: rustc_interface::passes::QueryContext::enter
  36: rustc_interface::queries::Queries::ongoing_codegen
  37: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  38: rustc_span::with_source_map
  39: rustc_interface::interface::create_compiler_and_run
  40: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
