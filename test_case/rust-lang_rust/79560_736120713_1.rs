
stack backtrace:
   0: rust_begin_unwind
             at /rustc/349b3b324dade7ca638091db93ba08bbc443c63d/library/std/src/panicking.rs:493:5
   1: std::panicking::begin_panic_fmt
             at /rustc/349b3b324dade7ca638091db93ba08bbc443c63d/library/std/src/panicking.rs:435:5
   2: rustc_query_system::query::plumbing::get_query_impl
   3: rustc_infer::traits::util::transitive_bounds_that_define_assoc_type
   4: <dyn rustc_typeck::astconv::AstConv>::associated_path_to_ty
   5: <dyn rustc_typeck::astconv::AstConv>::ast_ty_to_ty_inner
   6: <<dyn rustc_typeck::astconv::AstConv>::create_substs_for_ast_path::SubstsForAstPathCtxt as rustc_typeck::astconv::CreateSubstsForGenericArgsCtxt>::provided_kind
   7: <dyn rustc_typeck::astconv::AstConv>::create_substs_for_ast_path
   8: <dyn rustc_typeck::astconv::AstConv>::ast_path_substs_for_ty
   9: <dyn rustc_typeck::astconv::AstConv>::res_to_ty
  10: <dyn rustc_typeck::astconv::AstConv>::ast_ty_to_ty_inner
  11: <dyn rustc_typeck::astconv::AstConv>::ty_of_fn
  12: rustc_typeck::collect::fn_sig
  13: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::fn_sig>::compute
  14: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  15: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  16: rustc_data_structures::stack::ensure_sufficient_stack
  17: rustc_query_system::query::plumbing::get_query_impl
  18: rustc_query_system::query::plumbing::ensure_query_impl
  19: <rustc_typeck::collect::CollectItemTypesVisitor as rustc_hir::intravisit::Visitor>::visit_impl_item
  20: rustc_middle::hir::map::Map::visit_item_likes_in_module
  21: rustc_typeck::collect::collect_mod_item_types
  22: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::collect_mod_item_types>::compute
  23: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  24: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task
  25: rustc_data_structures::stack::ensure_sufficient_stack
  26: rustc_query_system::query::plumbing::get_query_impl
  27: rustc_query_system::query::plumbing::ensure_query_impl
  28: rustc_typeck::check_crate
  29: rustc_interface::passes::analysis
  30: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
  31: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  32: rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task
  33: rustc_data_structures::stack::ensure_sufficient_stack
  34: rustc_query_system::query::plumbing::get_query_impl
  35: rustc_interface::passes::QueryContext::enter
  36: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  37: rustc_span::with_source_map
  38: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.50.0-nightly (349b3b324 2020-11-29) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental -C link-arg=-fuse-ld=lld --crate-type lib

note: some of the compiler flags provided by cargo are hidden
