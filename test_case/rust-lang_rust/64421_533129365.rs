
error: internal compiler error: src/librustc/ty/context.rs:515: node_type: no type for node `expr 5 (hir_id=HirId { owner: DefIndex(228), local_id: 255 })`

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:643:9
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
             at src/libcore/fmt/mod.rs:1030
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1412
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:65
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:50
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:200
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:214
  10: rustc::util::common::panic_hook
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:481
  12: std::panicking::begin_panic
  13: rustc_errors::Handler::bug
  14: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  15: rustc::ty::context::tls::with_opt::{{closure}}
  16: rustc::ty::context::tls::with_context_opt
  17: rustc::ty::context::tls::with_opt
  18: rustc::util::bug::opt_span_bug_fmt
  19: rustc::util::bug::bug_fmt
  20: rustc::ty::context::TypeckTables::node_type::{{closure}}
  21: rustc::ty::context::TypeckTables::expr_ty
  22: clippy_lints::consts::ConstEvalLateContext::expr
  23: clippy_lints::consts::constant_simple
  24: clippy_lints::utils::hir_utils::SpanlessHash::hash_expr
  25: clippy_lints::utils::hir_utils::SpanlessHash::hash_tykind
  26: clippy_lints::utils::hir_utils::SpanlessHash::hash_expr
  27: clippy_lints::utils::hir_utils::SpanlessHash::hash_expr
  28: clippy_lints::utils::hir_utils::SpanlessHash::hash_block
  29: clippy_lints::utils::hir_utils::SpanlessHash::hash_expr
  30: clippy_lints::utils::hir_utils::SpanlessHash::hash_expr
  31: clippy_lints::utils::hir_utils::SpanlessHash::hash_expr
  32: clippy_lints::utils::hir_utils::SpanlessHash::hash_block
  33: clippy_lints::utils::hir_utils::SpanlessHash::hash_expr
  34: <clippy_lints::copies::CopyAndPaste as rustc::lint::LateLintPass>::check_expr
  35: <rustc::lint::context::LateLintPassObjects as rustc::lint::LateLintPass>::check_expr
  36: <rustc::lint::context::LateContextAndPass<T> as rustc::hir::intravisit::Visitor>::visit_expr
  37: rustc::hir::intravisit::walk_expr
  38: <rustc::lint::context::LateContextAndPass<T> as rustc::hir::intravisit::Visitor>::visit_expr
  39: rustc::hir::intravisit::walk_arm
  40: rustc::hir::intravisit::walk_expr
  41: <rustc::lint::context::LateContextAndPass<T> as rustc::hir::intravisit::Visitor>::visit_expr
  42: rustc::hir::intravisit::walk_expr
  43: <rustc::lint::context::LateContextAndPass<T> as rustc::hir::intravisit::Visitor>::visit_expr
  44: <rustc::lint::context::LateContextAndPass<T> as rustc::hir::intravisit::Visitor>::visit_nested_body
  45: <rustc::lint::context::LateContextAndPass<T> as rustc::hir::intravisit::Visitor>::visit_fn
  46: rustc::hir::intravisit::walk_impl_item
  47: rustc::hir::intravisit::Visitor::visit_nested_impl_item
  48: rustc::hir::intravisit::walk_item
  49: rustc::hir::intravisit::Visitor::visit_nested_item
  50: rustc::hir::intravisit::walk_item
  51: rustc::hir::intravisit::Visitor::visit_nested_item
  52: rustc::hir::intravisit::walk_crate
  53: rustc::lint::context::late_lint_pass_crate
  54: rustc::lint::context::late_lint_crate
  55: rustc::util::common::time
  56: rustc::util::common::time
  57: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:80
  58: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  59: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:80
  60: rustc_interface::passes::analysis::{{closure}}
  61: rustc::util::common::time
  62: rustc_interface::passes::analysis
  63: rustc::ty::query::__query_compute::analysis
  64: rustc::dep_graph::graph::DepGraph::with_task_impl
  65: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  66: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  67: rustc_interface::passes::create_global_ctxt::{{closure}}
  68: rustc_interface::interface::run_compiler_in_existing_thread_pool
  69: std::thread::local::LocalKey<T>::with
  70: scoped_tls::ScopedKey<T>::set
  71: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.39.0-nightly (eb48d6bde 2019-09-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `mysql_common`.
