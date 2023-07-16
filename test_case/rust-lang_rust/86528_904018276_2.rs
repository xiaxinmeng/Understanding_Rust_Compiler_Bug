
stack backtrace:
   0:        0x10626b431 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h48a34cf9e392861a
   1:        0x1062bb3ed - core::fmt::write::h8160330c41daaf61
   2:        0x10625cf7a - std::io::Write::write_fmt::h96cef022ec8f7c98
   3:        0x10626e845 - std::panicking::default_hook::{{closure}}::h0e535124ce564aa4
   4:        0x10626e42c - std::panicking::default_hook::ha71ede6560618eb7
   5:        0x10b962708 - rustc_driver::DEFAULT_HOOK::{{closure}}::{{closure}}::h368c83eb55044bb7
   6:        0x10626f086 - std::panicking::rust_panic_with_hook::h0972190e91792801
   7:        0x10626eafe - std::panicking::begin_panic_handler::{{closure}}::h01a54edb492b70d4
   8:        0x10626b8c7 - std::sys_common::backtrace::__rust_end_short_backtrace::hc48a9aab75427fc4
   9:        0x10626ea6a - _rust_begin_unwind
  10:        0x1062e654f - core::panicking::panic_fmt::h2e3306ce37bd7247
  11:        0x10fe79c9a - rustc_target::abi::FieldsShape::offset::hfd4cf83702c75039
  12:        0x10fbcc3c7 - <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold::h2cbd7a737c59b179
  13:        0x10fbc7a63 - <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter::h79ffd67a13d3a9d6
  14:        0x10fbecebe - rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>::record_layout_for_printing_outlined::{{closure}}::h8e536359c823a4ee
  15:        0x10fbee09a - <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt> as rustc_target::abi::LayoutOf>::layout_of::hbd28d2902a701d18
  16:        0x10bbf279d - core::ops::function::impls::<impl core::ops::function::FnMut<A> for &mut F>::call_mut::h18e176ba2ab183ee
  17:        0x10bbfb8fb - <core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::fold::h799903408cddae55
  18:        0x10bb6dbf3 - <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter::h2d63cf092896c881
  19:        0x10bc11665 - <rustc_target::abi::call::FnAbi<&rustc_middle::ty::TyS> as rustc_middle::ty::layout::FnAbiExt<C>>::new_internal::h6b4ac1529e739edb
  20:        0x10bc11008 - <rustc_target::abi::call::FnAbi<&rustc_middle::ty::TyS> as rustc_middle::ty::layout::FnAbiExt<C>>::of_instance::h59d454373b5cacaa
  21:        0x10bbc6d8e - rustc_codegen_llvm::mono_item::<impl rustc_codegen_ssa::traits::declare::PreDefineMethods for rustc_codegen_llvm::context::CodegenCx>::predefine_fn::h6baa1d862020acd8
  22:        0x10bbfa96a - rustc_codegen_llvm::base::compile_codegen_unit::module_codegen::h4899f239f9899df6
  23:        0x10bbcd0a7 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task::h6f2b5464c6200537
  24:        0x10bbfa26a - rustc_codegen_llvm::base::compile_codegen_unit::hacdf15d8b3940747
  25:        0x10bb9d467 - rustc_codegen_ssa::base::codegen_crate::h742d61e022cd1ec7
  26:        0x10bbe4560 - <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::h05fb9130036d6c8a
  27:        0x10ba8104d - rustc_interface::queries::Queries::ongoing_codegen::h46c464fb5cff49c4
  28:        0x10b9a22cc - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::h14dfcdb394aff575
  29:        0x10b96dc8e - rustc_span::with_source_map::h5e4953ed546e8a02
  30:        0x10b99fa36 - scoped_tls::ScopedKey<T>::set::h14ee411341c0e6ff
  31:        0x10b96ea29 - std::sys_common::backtrace::__rust_begin_short_backtrace::hee28e6cddabf9af9
  32:        0x10b965b6d - core::ops::function::FnOnce::call_once{{vtable.shim}}::h0c7b72c6ba13b917
  33:        0x10627a507 - std::sys::unix::thread::Thread::new::thread_start::ha97bc2d64cfc5adc
  34:     0x7fff204138fc - __pthread_start
