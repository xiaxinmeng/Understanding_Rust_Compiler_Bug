
{"message":"src/librustc/ty/context.rs:211: node type <B>::Item (hir_id=HirId { owner: DefIndex(246), local_id: 15 }) with HirId::owner DefId(0:246 ~ rayon[6715]::iter[0]::chain[0]::{{impl}}[2]::with_producer[0]::{{impl}}[0]) cannot be placed in TypeckTables with local_id_root DefId(0:238 ~ rayon[6715]::iter[0]::chain[0]::{{impl}}[2]::with_producer[0])","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"\u001b[0m\u001b[1m\u001b[38;5;9merror: internal compiler error\u001b[0m\u001b[0m\u001b[1m: src/librustc/ty/context.rs:211: node type <B>::Item (hir_id=HirId { owner: DefIndex(246), local_id: 15 }) with HirId::owner DefId(0:246 ~ rayon[6715]::iter[0]::chain[0]::{{impl}}[2]::with_producer[0]::{{impl}}[0]) cannot be placed in TypeckTables with local_id_root DefId(0:238 ~ rayon[6715]::iter[0]::chain[0]::{{impl}}[2]::with_producer[0])\u001b[0m\n\n"}
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:925:9
stack backtrace:
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: std::io::Write::write_fmt
   3: std::panicking::default_hook::{{closure}}
   4: std::panicking::default_hook
   5: rustc_driver::report_ice
   6: std::panicking::rust_panic_with_hook
   7: std::panicking::begin_panic
   8: rustc_errors::HandlerInner::bug
   9: rustc_errors::Handler::bug
  10: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  11: rustc::ty::context::tls::with_opt::{{closure}}
  12: rustc::ty::context::tls::with_context_opt
  13: rustc::ty::context::tls::with_opt
  14: rustc::util::bug::opt_span_bug_fmt
  15: rustc::util::bug::bug_fmt
  16: rustc::ty::context::validate_hir_id_for_typeck_tables::{{closure}}
  17: rustc::ty::context::tls::with::{{closure}}
  18: rustc::ty::context::tls::with_context::{{closure}}
  19: rustc::ty::context::tls::with_context_opt
  20: rustc::ty::context::tls::with_context
  21: rustc::ty::context::tls::with
  22: rustc::ty::context::TypeckTables::qpath_res
  23: rustc_save_analysis::SaveContext::get_path_res
  24: <rustc_save_analysis::dump_visitor::DumpVisitor as syntax::visit::Visitor>::visit_ty
  25: rustc_save_analysis::dump_visitor::DumpVisitor::process_path
  26: <rustc_save_analysis::dump_visitor::DumpVisitor as syntax::visit::Visitor>::visit_item
  27: rustc_save_analysis::dump_visitor::DumpVisitor::process_method
  28: <rustc_save_analysis::dump_visitor::DumpVisitor as syntax::visit::Visitor>::visit_item
  29: <rustc_save_analysis::dump_visitor::DumpVisitor as syntax::visit::Visitor>::visit_item
  30: <rustc_save_analysis::dump_visitor::DumpVisitor as syntax::visit::Visitor>::visit_item
  31: <rustc_save_analysis::dump_visitor::DumpVisitor as syntax::visit::Visitor>::visit_mod
  32: rustc::dep_graph::graph::DepGraph::with_ignore
  33: rustc_driver::run_compiler::{{closure}}::{{closure}}::{{closure}}
  34: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  35: rustc_interface::passes::create_global_ctxt::{{closure}}
  36: rustc_interface::interface::run_compiler_in_existing_thread_pool
  37: std::thread::local::LocalKey<T>::with
  38: scoped_tls::ScopedKey<T>::set
  39: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.40.0-nightly (518deda77 2019-10-18) running on x86_64-apple-darwin

note: compiler flags: -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack

