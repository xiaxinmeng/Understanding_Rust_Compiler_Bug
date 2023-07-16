
{"message":"src/librustc/ty/context.rs:211: node type <G>::NodeId (hir_id=HirId { owner: DefIndex(3567), local_id: 11 }) with HirId::owner DefId(0:3567 ~ petgraph[7dae]::algo[0]::tarjan_scc[0]::{{impl}}[3]) cannot be placed in TypeckTables with local_id_root DefId(0:980 ~ petgraph[7dae]::algo[0]::tarjan_scc[0])","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"\u001b[0m\u001b[1m\u001b[38;5;9merror: internal compiler error\u001b[0m\u001b[0m\u001b[1m: src/librustc/ty/context.rs:211: node type <G>::NodeId (hir_id=HirId { owner: DefIndex(3567), local_id: 11 }) with HirId::owner DefId(0:3567 ~ petgraph[7dae]::algo[0]::tarjan_scc[0]::{{impl}}[3]) cannot be placed in TypeckTables with local_id_root DefId(0:980 ~ petgraph[7dae]::algo[0]::tarjan_scc[0])\u001b[0m\n\n"}
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:925:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:77
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:61
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1028
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1412
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:65
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:50
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:189
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:206
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:473
  12: std::panicking::begin_panic
  13: rustc_errors::HandlerInner::bug
  14: rustc_errors::Handler::bug
  15: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  16: rustc::ty::context::tls::with_opt::{{closure}}
  17: rustc::ty::context::tls::with_context_opt
  18: rustc::ty::context::tls::with_opt
  19: rustc::util::bug::opt_span_bug_fmt
  20: rustc::util::bug::bug_fmt
  21: rustc::ty::context::validate_hir_id_for_typeck_tables::{{closure}}
  22: rustc::ty::context::tls::with::{{closure}}
  23: rustc::ty::context::tls::with_context::{{closure}}
  24: rustc::ty::context::tls::with_context_opt
  25: rustc::ty::context::tls::with_context
  26: rustc::ty::context::tls::with
  27: rustc::ty::context::TypeckTables::qpath_res
  28: rustc_save_analysis::SaveContext::get_path_res
  29: <rustc_save_analysis::dump_visitor::DumpVisitor as syntax::visit::Visitor>::visit_ty
  30: <rustc_save_analysis::dump_visitor::DumpVisitor as syntax::visit::Visitor>::visit_generics
  31: rustc_save_analysis::dump_visitor::DumpVisitor::process_generic_params
  32: <rustc_save_analysis::dump_visitor::DumpVisitor as syntax::visit::Visitor>::visit_item
  33: <rustc_save_analysis::dump_visitor::DumpVisitor as syntax::visit::Visitor>::visit_item
  34: <rustc_save_analysis::dump_visitor::DumpVisitor as syntax::visit::Visitor>::visit_item
  35: <rustc_save_analysis::dump_visitor::DumpVisitor as syntax::visit::Visitor>::visit_mod
  36: rustc::dep_graph::graph::DepGraph::with_ignore
  37: rustc_driver::run_compiler::{{closure}}::{{closure}}::{{closure}}
  38: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  39: rustc_interface::passes::create_global_ctxt::{{closure}}
  40: rustc_interface::interface::run_compiler_in_existing_thread_pool
  41: std::thread::local::LocalKey<T>::with
  42: scoped_tls::ScopedKey<T>::set
  43: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.40.0-nightly (fa0f7d008 2019-10-17) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
