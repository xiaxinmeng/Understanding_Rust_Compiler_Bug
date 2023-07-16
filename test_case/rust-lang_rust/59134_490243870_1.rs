
▶ RUST_BACKTRACE=1 cargo +nightly rustc --lib -- -Zsave-analysis
   Compiling rlsrepro v0.1.0 (/Users/swgillespie/Documents/workspace/rust/repros/rlsrepro)
error[E0425]: cannot find value `F` in this scope
  --> src/lib.rs:11:31
   |
11 |             const FLAG: u32 = F.bits;
   |                               ^ not found in this scope

error: internal compiler error: src/librustc/ty/context.rs:565: node_type: no type for node `expr F (hir_id=HirId { owner: DefIndex(0:10), local_id: 2 })`

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:643:9
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
  17: <rustc_save_analysis::dump_visitor::DumpVisitor<O> as syntax::visit::Visitor>::visit_expr
  18: rustc_save_analysis::dump_visitor::DumpVisitor<O>::process_assoc_const
  19: <rustc_save_analysis::dump_visitor::DumpVisitor<O> as syntax::visit::Visitor>::visit_item
  20: rustc_save_analysis::dump_visitor::DumpVisitor<O>::process_method
  21: <rustc_save_analysis::dump_visitor::DumpVisitor<O> as syntax::visit::Visitor>::visit_item
  22: <rustc_save_analysis::dump_visitor::DumpVisitor<O> as syntax::visit::Visitor>::visit_mod
  23: <rustc_save_analysis::DumpHandler as rustc_save_analysis::SaveHandler>::save
  24: rustc::dep_graph::graph::DepGraph::with_ignore
  25: rustc_driver::run_compiler::{{closure}}::{{closure}}::{{closure}}
  26: rustc::util::common::time
  27: rustc::ty::context::tls::enter_global
  28: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  29: rustc_interface::passes::create_global_ctxt::{{closure}}
  30: rustc_interface::passes::BoxedGlobalCtxt::enter
  31: rustc_interface::interface::run_compiler_in_existing_thread_pool
  32: std::thread::local::LocalKey<T>::with
  33: scoped_tls::ScopedKey<T>::set
  34: syntax::with_globals
query stack during panic:
end of query stack
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0425`.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.36.0-nightly (a19cf18c7 2019-05-06) running on x86_64-apple-darwin

note: compiler flags: -Z save-analysis -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `rlsrepro`.

To learn more, run the command again with --verbose.
