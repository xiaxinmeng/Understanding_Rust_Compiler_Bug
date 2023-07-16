
▶ RUST_BACKTRACE=1 rustc +dev -Z save-analysis --crate-type lib src/lib.rs
error[E0425]: cannot find value `bogus` in this scope
  --> src/lib.rs:26:27
   |
26 |         const FLAG: u32 = bogus.baz;
   |                           ^^^^^ not found in this scope

error: internal compiler error: src/librustc/ty/context.rs:262: node expr bogus.baz (hir_id=HirId { owner: DefIndex(0:8), local_id: 3 }) with HirId::owner DefId(0/0:8 ~ lib[8787]::bar[0]::{{impl}}[0]::FLAG[0]) cannot be placed in TypeckTables with local_id_root DefId(0/0:3 ~ lib[8787]::bar[0])

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:637:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::print
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
  14: rustc::ty::context::validate_hir_id_for_typeck_tables::{{closure}}
  15: rustc::ty::context::tls::with::{{closure}}
  16: rustc::ty::context::tls::with_context::{{closure}}
  17: rustc::ty::context::tls::with_context_opt
  18: rustc::ty::context::tls::with_context
  19: rustc::ty::context::tls::with
  20: rustc::ty::context::TypeckTables::expr_adjustments
  21: rustc::ty::context::TypeckTables::expr_ty_adjusted_opt
  22: rustc_save_analysis::SaveContext::get_expr_data
  23: <rustc_save_analysis::dump_visitor::DumpVisitor<O> as syntax::visit::Visitor>::visit_expr
  24: rustc_save_analysis::dump_visitor::DumpVisitor<O>::process_assoc_const
  25: <rustc_save_analysis::dump_visitor::DumpVisitor<O> as syntax::visit::Visitor>::visit_item
  26: <rustc_save_analysis::dump_visitor::DumpVisitor<O> as syntax::visit::Visitor>::visit_item
  27: <rustc_save_analysis::dump_visitor::DumpVisitor<O> as syntax::visit::Visitor>::visit_mod
  28: syntax::visit::walk_crate
  29: <rustc_save_analysis::DumpHandler as rustc_save_analysis::SaveHandler>::save
  30: rustc::dep_graph::graph::DepGraph::with_ignore
  31: rustc_save_analysis::process_crate
  32: rustc_driver::run_compiler::{{closure}}::{{closure}}::{{closure}}
  33: rustc::util::common::time
  34: rustc::ty::context::tls::enter_global
  35: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  36: rustc_interface::passes::create_global_ctxt::{{closure}}
  37: rustc_interface::passes::BoxedGlobalCtxt::enter
  38: rustc_interface::interface::run_compiler_in_existing_thread_pool
  39: std::thread::local::LocalKey<T>::with
  40: scoped_tls::ScopedKey<T>::set
  41: syntax::with_globals
query stack during panic:
end of query stack
