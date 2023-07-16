
error: internal compiler error: Encountered error `Unimplemented` selecting `Binder(<Qiz<u32> as Plugh<u32>>)` during codegen

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:977:27
stack backtrace:
   0: std::panicking::begin_panic
   1: rustc_errors::HandlerInner::emit_diagnostic
   2: rustc_errors::HandlerInner::emit_diag_at_span
   3: rustc_errors::HandlerInner::span_bug
   4: rustc_errors::Handler::delay_span_bug
   5: rustc_infer::infer::InferCtxtBuilder::enter
   6: rustc_trait_selection::traits::codegen::codegen_fulfill_obligation
   7: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::codegen_fulfill_obligation>::compute
   8: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
   9: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  10: rustc_data_structures::stack::ensure_sufficient_stack
  11: rustc_query_system::query::plumbing::get_query_impl
  12: rustc_ty::instance::inner_resolve_instance
  13: rustc_ty::instance::resolve_instance
  14: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::resolve_instance>::compute
  15: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  16: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  17: rustc_data_structures::stack::ensure_sufficient_stack
  18: rustc_query_system::query::plumbing::get_query_impl
  19: rustc_middle::ty::instance::Instance::resolve_opt_const_arg
  20: rustc_middle::ty::instance::Instance::resolve
  21: <rustc_mir::monomorphize::collector::MirNeighborCollector as rustc_middle::mir::visit::Visitor>::visit_terminator
  22: rustc_mir::monomorphize::collector::collect_neighbours
  23: rustc_mir::monomorphize::collector::collect_items_rec
  24: rustc_session::utils::<impl rustc_session::session::Session>::time
  25: rustc_mir::monomorphize::collector::collect_crate_mono_items
  26: rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items
  27: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::collect_and_partition_mono_items>::compute
  28: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  29: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  30: rustc_data_structures::stack::ensure_sufficient_stack
  31: rustc_query_system::query::plumbing::get_query_impl
  32: rustc_codegen_ssa::back::symbol_export::exported_symbols_provider_local
  33: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::exported_symbols>::compute
  34: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  35: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  36: rustc_data_structures::stack::ensure_sufficient_stack
  37: rustc_query_system::query::plumbing::get_query_impl
  38: rustc_metadata::rmeta::encoder::EncodeContext::encode_crate_root
  39: rustc_metadata::rmeta::encoder::encode_metadata_impl
  40: rustc_data_structures::sync::join
  41: rustc_metadata::rmeta::decoder::cstore_impl::<impl rustc_middle::middle::cstore::CrateStore for rustc_metadata::creader::CStore>::encode_metadata
  42: rustc_middle::ty::context::TyCtxt::encode_metadata
  43: rustc_interface::passes::QueryContext::enter
  44: rustc_interface::queries::Queries::ongoing_codegen
  45: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  46: rustc_span::with_source_map
  47: rustc_interface::interface::create_compiler_and_run
  48: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.49.0-nightly (ffa2e7ae8 2020-10-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z treat-err-as-bug -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [codegen_fulfill_obligation] checking if `Plugh` fulfills its obligations
#1 [resolve_instance] resolving instance `<Qiz<u32> as Plugh<u32>>::baz`
#2 [collect_and_partition_mono_items] collect_and_partition_mono_items
#3 [exported_symbols] exported_symbols
end of query stack
