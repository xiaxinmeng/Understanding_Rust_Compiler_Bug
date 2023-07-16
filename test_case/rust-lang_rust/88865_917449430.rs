
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1147:9
stack backtrace:
   0: std::panicking::begin_panic
   1: std::panic::panic_any
   2: rustc_errors::HandlerInner::bug
   3: rustc_errors::Handler::bug
   4: rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}
   5: rustc_middle::ty::context::tls::with_opt::{{closure}}
   6: rustc_middle::ty::context::tls::with_opt
   7: rustc_middle::util::bug::opt_span_bug_fmt
   8: rustc_middle::util::bug::bug_fmt
   9: rustc_middle::ich::impls_ty::<impl rustc_data_structures::stable_hasher::HashStable<rustc_middle::ich::hcx::StableHashingContext> for rustc_middle::ty::sty::RegionKind>::hash_stable
  10: rustc_middle::ty::sty::_DERIVE_rustc_data_structures_stable_hasher_HashStable_rustc_middle_ich_StableHashingContext_ctx_FOR_TyKind::<impl rustc_data_structures::stable_hasher::HashStable<rustc_middle::ich::hcx::StableHashingContext> for rustc_middle::ty::sty::TyKind>::hash_stable
  11: <T as rustc_query_system::dep_graph::dep_node::DepNodeParams<Ctxt>>::to_fingerprint
  12: rustc_query_system::dep_graph::dep_node::DepNode<K>::construct
  13: rustc_query_system::query::plumbing::get_query_impl
  14: rustc_query_system::query::plumbing::get_query
  15: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::type_uninhabited_from
  16: rustc_middle::ty::inhabitedness::<impl rustc_middle::ty::TyS>::uninhabited_from
  17: rustc_middle::ty::inhabitedness::<impl rustc_middle::ty::context::TyCtxt>::is_ty_uninhabited_from
  18: rustc_typeck::check::generator_interior::check_must_not_suspend_ty
  19: rustc_typeck::check::generator_interior::InteriorVisitor::record
  20: <rustc_typeck::check::generator_interior::InteriorVisitor as rustc_hir::intravisit::Visitor>::visit_pat
  21: rustc_hir::intravisit::walk_block
  22: <rustc_typeck::check::generator_interior::InteriorVisitor as rustc_hir::intravisit::Visitor>::visit_expr
  23: rustc_typeck::check::generator_interior::resolve_interior
  24: rustc_typeck::check::fn_ctxt::_impl::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::resolve_generator_interiors
  25: rustc_infer::infer::InferCtxtBuilder::enter
  ...
  