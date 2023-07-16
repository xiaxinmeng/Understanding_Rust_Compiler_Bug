
   Compiling no_main v0.1.0 (/Users/ekuber/workspace/asdf)
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: DistinctSources(DistinctSources { begin: (Real("src/main.rs"), BytePos(0)), end: (Macros("::quick_error::quick_error"), BytePos(7657333)) })', src/librustc_errors/lib.rs:197:29
stack backtrace:
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: std::io::Write::write_fmt
   3: std::sys_common::backtrace::print
   4: std::panicking::default_hook::{{closure}}
   5: std::panicking::default_hook
   6: rustc_driver::report_ice
   7: std::panicking::rust_panic_with_hook
   8: rust_begin_unwind
   9: core::panicking::panic_fmt
  10: core::result::unwrap_failed
  11: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &mut F>::call_once
  12: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter
  13: rustc_errors::CodeSuggestion::splice_lines
  14: <rustc_errors::emitter::EmitterWriter as rustc_errors::emitter::Emitter>::emit_diagnostic
  15: <rustc_errors::json::JsonEmitter as rustc_errors::emitter::Emitter>::emit_diagnostic
  16: rustc_errors::HandlerInner::emit_diagnostic
  17: rustc_errors::Handler::emit_diagnostic
  18: rustc_errors::diagnostic_builder::DiagnosticBuilder::emit
  19: rustc_resolve::lifetimes::LifetimeContext::resolve_elided_lifetimes
  20: <rustc_resolve::lifetimes::LifetimeContext as rustc_hir::intravisit::Visitor>::visit_lifetime
  21: <rustc_resolve::lifetimes::LifetimeContext as rustc_hir::intravisit::Visitor>::visit_ty
  22: rustc_hir::intravisit::walk_struct_def
  23: rustc_hir::intravisit::walk_variant
  24: rustc_hir::intravisit::walk_item
  25: rustc_resolve::lifetimes::LifetimeContext::with
  26: <rustc_resolve::lifetimes::LifetimeContext as rustc_hir::intravisit::Visitor>::visit_item
  27: rustc_resolve::lifetimes::resolve_lifetimes
  28: rustc::ty::query::__query_compute::resolve_lifetimes
  29: rustc::dep_graph::graph::DepGraph::with_task_impl
  30: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  31: core::ops::function::FnOnce::call_once
  32: rustc::dep_graph::graph::DepGraph::with_task_impl
  33: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  34: rustc::ty::context::TyCtxt::object_lifetime_defaults
  35: rustc_typeck::collect::generics_of
  36: rustc::ty::query::__query_compute::generics_of
  37: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::generics_of>::compute
  38: rustc::dep_graph::graph::DepGraph::with_task_impl
  39: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  40: <rustc_typeck::collect::CollectItemTypesVisitor as rustc_hir::intravisit::Visitor>::visit_item
  41: rustc::hir::map::Map::visit_item_likes_in_module
  42: rustc_typeck::collect::collect_mod_item_types
  43: rustc::ty::query::__query_compute::collect_mod_item_types
  44: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::collect_mod_item_types>::compute
  45: rustc::dep_graph::graph::DepGraph::with_task_impl
  46: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  47: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::ensure_query
  48: rustc_session::session::Session::track_errors
  49: rustc_typeck::check_crate
  50: rustc_interface::passes::analysis
  51: rustc::ty::query::__query_compute::analysis
  52: rustc::dep_graph::graph::DepGraph::with_eval_always_task
  53: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  54: rustc::ty::context::tls::enter_global
  55: rustc_interface::interface::run_compiler_in_existing_thread_pool
  56: scoped_tls::ScopedKey<T>::set
  57: syntax::with_globals
