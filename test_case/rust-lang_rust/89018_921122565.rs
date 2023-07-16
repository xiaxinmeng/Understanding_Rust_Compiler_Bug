
thread 'rustc' panicked at 'Failed to recover key for crate_hash(7b325c0bcf1c8f7e-78e92a3d885642c8) with hash 7b325c0bcf1c8f7e-78e92a3d885642c8', compiler/rustc_query_impl/src/lib.rs:54:1
stack backtrace:
   0: rust_begin_unwind
             at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b/library/std/src/panicking.rs:515:5
   1: std::panicking::begin_panic_fmt
             at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b/library/std/src/panicking.rs:457:5
   2: rustc_query_impl::query_callbacks::is_no_builtins::try_load_from_on_disk_cache
   3: rustc_query_system::dep_graph::graph::DepGraph<K>::exec_cache_promotions
   4: rustc_middle::ty::context::tls::with_context::{{closure}}
   5: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
   6: <rustc_query_impl::on_disk_cache::OnDiskCache as rustc_middle::ty::context::OnDiskCache>::serialize
   7: rustc_middle::ty::context::TyCtxt::serialize_query_result_cache
   8: rustc_session::utils::<impl rustc_session::session::Session>::time
   9: rustc_incremental::persist::save::save_in
  10: rustc_session::utils::<impl rustc_session::session::Session>::time
  11: rustc_data_structures::sync::join
  12: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  13: rustc_incremental::persist::save::save_dep_graph
  14: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  15: rustc_span::with_source_map
  16: rustc_interface::interface::create_compiler_and_run
  17: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0 (c8dfcfe04 2021-09-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `storing_lists_vectors`
