
error: constant expression depends on a generic parameter
  |
  = note: this may fail depending on what value the parameter takes

error: internal compiler error: compiler/rustc_traits/src/normalize_erasing_regions.rs:37:32: could not fully normalize `<my_crate::Const<U> as my_crate::Trait>::AssocTy`

thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:945:9
stack backtrace:
   0: std::panicking::begin_panic
   1: rustc_errors::HandlerInner::bug
   2: rustc_errors::Handler::bug
   3: rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}
   4: rustc_middle::ty::context::tls::with_opt::{{closure}}
   5: rustc_middle::ty::context::tls::with_opt
   6: rustc_middle::util::bug::opt_span_bug_fmt
   7: rustc_middle::util::bug::bug_fmt
   8: rustc_infer::infer::InferCtxtBuilder::enter
   9: rustc_traits::normalize_erasing_regions::normalize_generic_arg_after_erasing_regions
  10: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  11: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  12: rustc_data_structures::stack::ensure_sufficient_stack
  13: rustc_query_system::query::plumbing::get_query_impl
  14: rustc_middle::ty::structural_impls::fold_list
  15: rustc_middle::ty::normalize_erasing_regions::<impl rustc_middle::ty::context::TyCtxt>::normalize_erasing_regions
  16: rustc_middle::ty::layout::<impl rustc_middle::ty::instance::Instance>::fn_sig_for_fn_abi
  17: <rustc_target::abi::call::FnAbi<&rustc_middle::ty::TyS> as rustc_middle::ty::layout::FnAbiExt<C>>::of_instance
  18: rustc_codegen_ssa::mir::block::<impl rustc_codegen_ssa::mir::FunctionCx<Bx>>::codegen_call_terminator
  19: rustc_codegen_ssa::mir::block::<impl rustc_codegen_ssa::mir::FunctionCx<Bx>>::codegen_block
  20: rustc_codegen_ssa::mir::codegen_mir
  21: rustc_codegen_ssa::base::codegen_instance
  22: <rustc_middle::mir::mono::MonoItem as rustc_codegen_ssa::mono_item::MonoItemExt>::define
  23: rustc_codegen_llvm::base::compile_codegen_unit::module_codegen
  24: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  25: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  26: rustc_codegen_llvm::base::compile_codegen_unit
  27: rustc_codegen_ssa::base::codegen_crate
  28: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
  29: rustc_session::utils::<impl rustc_session::session::Session>::time
  30: rustc_interface::queries::Queries::ongoing_codegen
  31: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  32: rustc_span::with_source_map
  33: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.49.0-nightly (e3051d8c2 2020-10-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [normalize_generic_arg_after_erasing_regions] normalizing `<my_crate::Const<U> as my_crate::Trait>::AssocTy`
end of query stack
error: aborting due to 2 previous errors

error: could not compile `my_crate`
