
$ RUST_BACKTRACE=1 rustc +nightly --edition=2018 src/main.rs --crate-type bin -C debuginfo=2
thread 'rustc' panicked at 'assertion failed: layout.abi.is_uninhabited()', compiler/rustc_middle/src/ty/layout.rs:228:21
stack backtrace:
   0: _rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::panic
   3: rustc_middle::ty::layout::layout_raw
   4: rustc_query_system::query::plumbing::get_query_impl
   5: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::layout_raw
   6: <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt> as rustc_target::abi::LayoutOf>::layout_of
   7: rustc_codegen_llvm::debuginfo::metadata::create_struct_stub
   8: rustc_codegen_llvm::debuginfo::metadata::prepare_tuple_metadata
   9: rustc_codegen_llvm::debuginfo::metadata::type_metadata
  10: core::ops::function::impls::<impl core::ops::function::FnMut<A> for &mut F>::call_mut
  11: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
  12: rustc_codegen_llvm::debuginfo::metadata::set_members_of_composite_type
  13: rustc_codegen_llvm::debuginfo::metadata::RecursiveTypeDescription::finalize
  14: rustc_codegen_llvm::debuginfo::metadata::type_metadata
  15: <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold
  16: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
  17: rustc_codegen_llvm::debuginfo::metadata::MemberDescriptionFactory::create_member_descriptions
  18: rustc_codegen_llvm::debuginfo::metadata::RecursiveTypeDescription::finalize
  19: rustc_codegen_llvm::debuginfo::metadata::type_metadata
  20: rustc_codegen_llvm::debuginfo::<impl rustc_codegen_ssa::traits::debuginfo::DebugInfoMethods for rustc_codegen_llvm::context::CodegenCx>::dbg_scope_fn::get_containing_scope
  21: rustc_codegen_llvm::debuginfo::<impl rustc_codegen_ssa::traits::debuginfo::DebugInfoMethods for rustc_codegen_llvm::context::CodegenCx>::dbg_scope_fn
  22: rustc_codegen_llvm::debuginfo::<impl rustc_codegen_ssa::traits::debuginfo::DebugInfoMethods for rustc_codegen_llvm::context::CodegenCx>::create_function_debug_context
  23: rustc_codegen_ssa::mir::codegen_mir
  24: rustc_codegen_ssa::base::codegen_instance
  25: <rustc_middle::mir::mono::MonoItem as rustc_codegen_ssa::mono_item::MonoItemExt>::define
  26: rustc_codegen_llvm::base::compile_codegen_unit::module_codegen
  27: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  28: rustc_codegen_llvm::base::compile_codegen_unit
  29: rustc_codegen_ssa::base::codegen_crate
  30: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
  31: rustc_interface::queries::Queries::ongoing_codegen
  32: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  33: rustc_span::with_source_map
  34: scoped_tls::ScopedKey<T>::set
