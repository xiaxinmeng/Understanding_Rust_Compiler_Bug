
error: internal compiler error: src/librustc/ty/context.rs:516: node_type: no type for node `expr 5 (hir_id=HirId { owner: DefIndex(228), local_id: 255 })`

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:644:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.34/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.34/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:47
   3: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:36
   4: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:200
   5: std::panicking::default_hook
             at src/libstd/panicking.rs:214
   6: rustc::util::common::panic_hook
   7: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:481
   8: std::panicking::begin_panic
   9: rustc_errors::Handler::bug
  10: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  11: rustc::ty::context::tls::with_opt::{{closure}}
  12: rustc::ty::context::tls::with_context_opt
  13: rustc::ty::context::tls::with_opt
  14: rustc::util::bug::opt_span_bug_fmt
  15: rustc::util::bug::bug_fmt
  16: rustc::ty::context::TypeckTables::node_type::{{closure}}
  17: rustc::ty::context::TypeckTables::expr_ty
  18: clippy_lints::consts::ConstEvalLateContext::expr
  19: clippy_lints::consts::constant_simple
  20: clippy_lints::utils::hir_utils::SpanlessHash::hash_expr
  21: clippy_lints::utils::hir_utils::SpanlessHash::hash_tykind
  22: clippy_lints::utils::hir_utils::SpanlessHash::hash_expr
  23: clippy_lints::utils::hir_utils::SpanlessHash::hash_expr
  24: clippy_lints::utils::hir_utils::SpanlessHash::hash_block
  25: clippy_lints::utils::hir_utils::SpanlessHash::hash_expr
  26: clippy_lints::utils::hir_utils::SpanlessHash::hash_expr
  27: clippy_lints::utils::hir_utils::SpanlessHash::hash_expr
  28: clippy_lints::utils::hir_utils::SpanlessHash::hash_block
  29: clippy_lints::utils::hir_utils::SpanlessHash::hash_expr
  30: <clippy_lints::copies::CopyAndPaste as rustc::lint::LateLintPass>::check_expr
  31: <rustc::lint::context::LateLintPassObjects as rustc::lint::LateLintPass>::check_expr
  32: <rustc::lint::context::LateContextAndPass<T> as rustc::hir::intravisit::Visitor>::visit_expr
  33: rustc::hir::intravisit::walk_expr
  34: <rustc::lint::context::LateContextAndPass<T> as rustc::hir::intravisit::Visitor>::visit_expr
  35: rustc::hir::intravisit::walk_arm
  36: rustc::hir::intravisit::walk_expr
  37: <rustc::lint::context::LateContextAndPass<T> as rustc::hir::intravisit::Visitor>::visit_expr
  38: rustc::hir::intravisit::walk_expr
  39: <rustc::lint::context::LateContextAndPass<T> as rustc::hir::intravisit::Visitor>::visit_expr
  40: <rustc::lint::context::LateContextAndPass<T> as rustc::hir::intravisit::Visitor>::visit_nested_body
  41: <rustc::lint::context::LateContextAndPass<T> as rustc::hir::intravisit::Visitor>::visit_fn
  42: rustc::hir::intravisit::walk_impl_item
  43: rustc::hir::intravisit::Visitor::visit_nested_impl_item
  44: rustc::hir::intravisit::walk_item
  45: rustc::hir::intravisit::Visitor::visit_nested_item
  46: rustc::hir::intravisit::walk_item
  47: rustc::hir::intravisit::Visitor::visit_nested_item
  48: rustc::hir::intravisit::walk_crate
  49: rustc::lint::context::late_lint_pass_crate
  50: rustc::lint::context::late_lint_crate
  51: rustc::util::common::time
  52: rustc::util::common::time
  53: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:80
  54: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  55: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:80
  56: rustc_interface::passes::analysis::{{closure}}
  57: rustc::util::common::time
  58: rustc_interface::passes::analysis
  59: rustc::ty::query::__query_compute::analysis
  60: rustc::dep_graph::graph::DepGraph::with_task_impl
  61: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  62: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  63: rustc_interface::passes::create_global_ctxt::{{closure}}
  64: rustc_interface::interface::run_compiler_in_existing_thread_pool
  65: std::thread::local::LocalKey<T>::with
  66: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.38.0 (625451e37 2019-09-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -C panic=abort -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `mysql_common`.
warning: build failed, waiting for other jobs to finish...
error: build failed
