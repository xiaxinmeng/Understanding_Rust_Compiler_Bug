
   0: rust_begin_unwind
             at /rustc/c7331d65bdbab1187f5a9b8f5b918248678ebdb9/library/std/src/panicking.rs:515:5
   1: core::panicking::panic_fmt
             at /rustc/c7331d65bdbab1187f5a9b8f5b918248678ebdb9/library/core/src/panicking.rs:92:14
   2: rustc_target::abi::FieldsShape::offset
   3: <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold
   4: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
   5: <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt> as rustc_target::abi::LayoutOf>::layout_of
   6: <rustc_codegen_llvm::context::CodegenCx as rustc_target::abi::LayoutOf>::spanned_layout_of
   7: core::ops::function::impls::<impl core::ops::function::FnMut<A> for &mut F>::call_mut
   8: <core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::fold
   9: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
  10: <rustc_target::abi::call::FnAbi<&rustc_middle::ty::TyS> as rustc_middle::ty::layout::FnAbiExt<C>>::new_internal
  11: <rustc_target::abi::call::FnAbi<&rustc_middle::ty::TyS> as rustc_middle::ty::layout::FnAbiExt<C>>::of_instance
  12: rustc_codegen_llvm::mono_item::<impl rustc_codegen_ssa::traits::declare::PreDefineMethods for rustc_codegen_llvm::context::CodegenCx>::predefine_fn
  13: rustc_codegen_llvm::base::compile_codegen_unit::module_codegen
  14: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  15: rustc_codegen_llvm::base::compile_codegen_unit
  16: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
  17: rustc_interface::passes::QueryContext::enter
  18: rustc_interface::queries::Queries::ongoing_codegen
  19: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  20: rustc_span::with_source_map
  21: rustc_interface::interface::create_compiler_and_run
  22: scoped_tls::ScopedKey<T>::set
