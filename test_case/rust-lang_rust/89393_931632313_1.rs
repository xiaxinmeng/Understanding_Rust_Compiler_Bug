
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1146:9
stack backtrace:
   0: std::panicking::begin_panic
   1: std::panic::panic_any
   2: rustc_errors::HandlerInner::bug
   3: rustc_errors::Handler::bug
   4: rustc_middle::ty::context::tls::with_opt
   5: rustc_middle::util::bug::opt_span_bug_fmt
   6: rustc_middle::util::bug::bug_fmt
   7: rustc_mir_build::thir::pattern::deconstruct_pat::Fields::wildcards
   8: rustc_mir_build::thir::pattern::deconstruct_pat::DeconstructedPat::specialize
   9: rustc_mir_build::thir::pattern::usefulness::is_useful
  10: rustc_mir_build::thir::pattern::usefulness::is_useful
  11: rustc_mir_build::thir::pattern::usefulness::compute_match_usefulness
  12: <rustc_mir_build::thir::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
  13: rustc_hir::intravisit::walk_expr
  14: <rustc_mir_build::thir::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
  15: rustc_mir_build::thir::pattern::check_match::check_match
  16: rustc_query_system::query::plumbing::try_execute_query
  17: rustc_query_system::query::plumbing::get_query
  18: rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::par_body_owners
  19: rustc_session::utils::<impl rustc_session::session::Session>::time
  20: rustc_session::utils::<impl rustc_session::session::Session>::time
  21: rustc_interface::passes::analysis
  22: rustc_query_system::query::plumbing::try_execute_query
  23: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
  24: rustc_interface::passes::QueryContext::enter
  25: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  26: rustc_span::with_source_map
  27: scoped_tls::ScopedKey<T>::set
