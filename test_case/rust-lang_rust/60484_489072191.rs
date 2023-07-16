
error: the item `libc` is imported redundantly
  --> src/mount.rs:64:9
   |
2  | use libc;
   |     ---- the item `libc` is already imported here
...
64 |     use libc;
   |         ^^^^
   |
note: lint level defined here
  --> src/lib.rs:13:9
   |
13 | #![deny(unused)]
   |         ^^^^^^
   = note: #[deny(unused_imports)] implied by #[deny(unused)]

error: internal compiler error: src/librustc/ty/context.rs:562: node_type: no type for node `expr <Self>::AT_SYMLINK_NOFOLLOW (id=45856)`

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:643:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
   1: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at src/libstd/sys_common/backtrace.rs:59
             at src/libstd/panicking.rs:197
   3: std::panicking::default_hook
             at src/libstd/panicking.rs:211
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:478
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
  22: <rustc_save_analysis::dump_visitor::DumpVisitor<O> as syntax::visit::Visitor>::visit_item
  23: <rustc_save_analysis::dump_visitor::DumpVisitor<O> as syntax::visit::Visitor>::visit_mod
  24: <rustc_save_analysis::DumpHandler as rustc_save_analysis::SaveHandler>::save
  25: rustc::dep_graph::graph::DepGraph::with_ignore
  26: rustc_driver::run_compiler::{{closure}}::{{closure}}::{{closure}}
  27: rustc::util::common::time
  28: rustc::ty::context::tls::enter_global
  29: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  30: rustc_interface::passes::create_global_ctxt::{{closure}}
  31: rustc_interface::interface::run_compiler_in_existing_thread_pool
  32: std::thread::local::LocalKey<T>::with
  33: scoped_tls::ScopedKey<T>::set
  34: syntax::with_globals
query stack during panic:
end of query stack
error: aborting due to 2 previous errors
