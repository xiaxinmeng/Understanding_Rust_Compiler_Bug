
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_middle/src/ty/query/mod.rs:235:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/std/src/panicking.rs:493:5
   1: core::panicking::panic_fmt
             at /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/core/src/panicking.rs:92:14
   2: core::panicking::panic
             at /rustc/3d6705aa5abffe94c83bf09af8c3ba3c599845fc/library/core/src/panicking.rs:50:5
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

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.50.0-nightly (3d6705aa5 2020-12-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
