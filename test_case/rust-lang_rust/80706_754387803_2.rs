
error: internal compiler error: compiler/rustc_codegen_ssa/src/debuginfo/type_names.rs:228:13: debuginfo: Trying to create type name for unexpected type: <SaveUser as StorageRequestReturnType>::Output

thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:958:9
stack backtrace:
   0: std::panicking::begin_panic
   1: rustc_errors::HandlerInner::bug
   2: rustc_errors::Handler::bug
   3: rustc_middle::ty::context::tls::with_opt
   4: rustc_middle::util::bug::opt_span_bug_fmt
   5: rustc_middle::util::bug::bug_fmt
   6: rustc_codegen_ssa::debuginfo::type_names::push_debuginfo_type_name
   7: rustc_codegen_ssa::debuginfo::type_names::push_debuginfo_type_name::push_type_params
   8: rustc_codegen_ssa::debuginfo::type_names::compute_debuginfo_type_name
   9: rustc_codegen_llvm::debuginfo::metadata::prepare_enum_metadata
  10: rustc_codegen_llvm::debuginfo::metadata::type_metadata
  11: rustc_codegen_llvm::debuginfo::<impl rustc_codegen_ssa::traits::debuginfo::DebugInfoMethods for rustc_codegen_llvm::context::CodegenCx>::create_dbg_var
  12: rustc_codegen_ssa::mir::codegen_mir
  13: rustc_codegen_ssa::base::codegen_instance
  14: <rustc_middle::mir::mono::MonoItem as rustc_codegen_ssa::mono_item::MonoItemExt>::define
  15: rustc_codegen_llvm::base::compile_codegen_unit::module_codegen
  16: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  17: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  18: rustc_codegen_llvm::base::compile_codegen_unit
  19: rustc_codegen_ssa::base::codegen_crate
  20: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
  21: rustc_session::utils::<impl rustc_session::session::Session>::time
  22: rustc_interface::passes::QueryContext::enter
  23: rustc_interface::queries::Queries::ongoing_codegen
  24: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  25: rustc_span::with_source_map
  26: rustc_interface::interface::create_compiler_and_run
  27: rustc_span::with_session_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

query stack during panic:
end of query stack
