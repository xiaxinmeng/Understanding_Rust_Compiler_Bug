
{"message":"src/librustc/ty/context.rs:541: node_type: no type for node `expr <Self>::STARTED (id=32431)`","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc/ty/context.rs:541: node_type: no type for node `expr <Self>::STARTED (id=32431)`\n\n"}
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:620:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
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
  14: rustc::ty::context::TypeckTables::node_type::{{closure}}
  15: rustc::ty::context::TypeckTables::expr_ty_adjusted
  16: rustc_save_analysis::SaveContext::get_expr_data
  17: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_expr
  18: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O>>::process_assoc_const
  19: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_item
  20: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O>>::process_method
  21: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_item
  22: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_item
  23: <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_mod
  24: <rustc_save_analysis::DumpHandler<'a> as rustc_save_analysis::SaveHandler>::save
  25: rustc::dep_graph::graph::DepGraph::with_ignore
  26: rustc_driver::enable_save_analysis::{{closure}}::{{closure}}
  27: rustc_driver::enable_save_analysis::{{closure}}
  28: rustc::dep_graph::graph::DepGraph::with_ignore
  29: rustc_driver::driver::compile_input::{{closure}}
  30: <std::thread::local::LocalKey<T>>::with
  31: rustc::ty::context::TyCtxt::create_and_enter
  32: rustc_driver::driver::compile_input
  33: rustc_driver::run_compiler_with_pool
  34: <scoped_tls::ScopedKey<T>>::set
  35: rustc_driver::run_compiler
  36: <scoped_tls::ScopedKey<T>>::set
query stack during panic:
end of query stack
