
thread 'rustc' panicked at 'Failed to recover key for type_of(44bb8dfde12fe5f2-84c94631772ec105) with hash 44bb8dfde12fe5f2-84c94631772ec105', compiler/rustc_middle/src/ty/query/mod.rs:235:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/d32c320d7eee56706486fef6be778495303afe9e/library/std/src/panicking.rs:493:5
   1: std::panicking::begin_panic_fmt
             at /rustc/d32c320d7eee56706486fef6be778495303afe9e/library/std/src/panicking.rs:435:5
   2: rustc_middle::ty::query::try_load_from_on_disk_cache::{{closure}}
   3: rustc_middle::ty::query::try_load_from_on_disk_cache
   4: rustc_query_system::dep_graph::graph::DepGraph<K>::exec_cache_promotions
   5: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
   6: rustc_incremental::persist::save::save_in
   7: rustc_data_structures::sync::join
   8: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
   9: rustc_incremental::persist::save::save_dep_graph
  10: rustc_codegen_ssa::base::finalize_tcx
  11: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
  12: rustc_session::utils::<impl rustc_session::session::Session>::time
  13: rustc_interface::passes::QueryContext::enter
  14: rustc_interface::queries::Queries::ongoing_codegen
  15: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  16: rustc_span::with_source_map
  17: rustc_interface::interface::create_compiler_and_run
  18: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
