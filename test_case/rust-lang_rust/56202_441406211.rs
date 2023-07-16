
error: internal compiler error: librustc/hir/def.rs:265: attempted .def_id() on invalid def:

thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:600:9
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::_print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::bug
   8: rustc::util::bug::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::util::bug::opt_span_bug_fmt
  13: rustc::util::bug::bug_fmt
  14: rustc::hir::def::Def::def_id::{{closure}}
  15: <rustc::middle::reachable::ReachableContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_expr
  16: rustc::hir::intravisit::walk_expr
  17: <rustc::middle::reachable::ReachableContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_expr
  18: <rustc::middle::reachable::ReachableContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_expr
  19: rustc::middle::reachable::reachable_set
  20: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::reachable_set<'tcx>>::compute
  21: rustc::ty::context::tls::with_context
  22: rustc::dep_graph::graph::DepGraph::with_task_impl
  23: rustc::ty::context::tls::with_related_context
  24: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  25: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  26: rustc_codegen_ssa::back::symbol_export::reachable_non_generics_provider
  27: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::reachable_non_generics<'tcx>>::compute
  28: rustc::ty::context::tls::with_context
  29: rustc::dep_graph::graph::DepGraph::with_task_impl
  30: rustc::ty::context::tls::with_related_context
  31: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  32: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  33: rustc_codegen_ssa::back::symbol_export::exported_symbols_provider_local
  34: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::exported_symbols<'tcx>>::compute
  35: rustc::ty::context::tls::with_context
  36: rustc::dep_graph::graph::DepGraph::with_task_impl
  37: rustc::ty::context::tls::with_related_context
  38: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  39: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  40: rustc_metadata::encoder::encode_metadata
  41: rustc_metadata::cstore_impl::<impl rustc::middle::cstore::CrateStore for rustc_metadata::cstore::CStore>::encode_metadata
  42: rustc::ty::context::TyCtxt::encode_metadata
  43: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::ExtraBackendMethods>::write_metadata
  44: rustc::util::common::time
  45: rustc_codegen_ssa::base::codegen_crate
  46: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  47: rustc::util::common::time
  48: rustc_driver::driver::phase_4_codegen
  49: rustc_driver::driver::compile_input::{{closure}}
  50: rustc::ty::context::tls::enter_context
  51: <std::thread::local::LocalKey<T>>::with
  52: rustc::ty::context::TyCtxt::create_and_enter
  53: rustc_driver::driver::compile_input
  54: rustc_driver::run_compiler_with_pool
  55: <scoped_tls::ScopedKey<T>>::set
  56: rustc_driver::run_compiler
  57: syntax::with_globals
  58: __rust_maybe_catch_panic
  59: rustc_driver::run
  60: rustc_driver::main
  61: std::rt::lang_start::{{closure}}
  62: std::panicking::try::do_call
  63: __rust_maybe_catch_panic
  64: std::rt::lang_start_internal
  65: main
query stack during panic:
#0 [reachable_set] reachability
#1 [reachable_non_generics] looking up the exported symbols of a crate
#2 [exported_symbols] exported_symbols
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.32.0-nightly (5aff30734 2018-11-19) running on x86_64-apple-darwin

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `test_indexing`.
