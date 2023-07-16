
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: DistinctSources(DistinctSources { begin: (Real("src/shell/filesystem_shell.rs"), BytePos(517740)), end: (Macros("::async_stream::stream"), BytePos(16954038)) })', src/librustc_errors/lib.rs:187:29
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:77
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:59
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1057
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1426
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:204
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:224
  10: rustc_driver::report_ice
  11: <alloc::boxed::Box<F> as core::ops::function::Fn<A>>::call
             at /rustc/3291ae33907f2a866ea6cea89113200555038d06/src/liballoc/boxed.rs:1029
  12: proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::{{closure}}
             at /rustc/3291ae33907f2a866ea6cea89113200555038d06/src/libproc_macro/bridge/client.rs:305
  13: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:476
  14: rust_begin_unwind
             at src/libstd/panicking.rs:380
  15: core::panicking::panic_fmt
             at src/libcore/panicking.rs:85
  16: core::option::expect_none_failed
             at src/libcore/option.rs:1198
  17: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::fold
  18: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T,I>>::from_iter
  19: <rustc_errors::emitter::EmitterWriter as rustc_errors::emitter::Emitter>::emit_diagnostic
  20: <rustc_errors::json::JsonEmitter as rustc_errors::emitter::Emitter>::emit_diagnostic
  21: rustc_errors::HandlerInner::emit_diagnostic
  22: rustc_errors::diagnostic_builder::DiagnosticBuilder::emit
  23: rustc_typeck::check::demand::<impl rustc_typeck::check::FnCtxt>::demand_coerce
  24: rustc_typeck::check::FnCtxt::check_argument_types
  25: rustc_typeck::check::FnCtxt::check_method_argument_types
  26: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  27: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  28: rustc_typeck::check::_match::<impl rustc_typeck::check::FnCtxt>::check_match
  29: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  30: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  31: rustc_typeck::check::_match::<impl rustc_typeck::check::FnCtxt>::check_match
  32: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  33: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  34: rustc_typeck::check::FnCtxt::check_block_with_expected
  35: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  36: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  37: rustc_typeck::check::_match::<impl rustc_typeck::check::FnCtxt>::check_match
  38: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  39: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  40: rustc_typeck::check::FnCtxt::check_block_with_expected
  41: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  42: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  43: rustc_typeck::check::FnCtxt::check_stmt
  44: rustc_typeck::check::FnCtxt::check_block_with_expected
  45: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  46: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  47: rustc_typeck::check::_match::<impl rustc_typeck::check::FnCtxt>::check_match
  48: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  49: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  50: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  51: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  52: rustc_typeck::check::FnCtxt::check_block_with_expected
  53: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  54: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  55: rustc_typeck::check::_match::<impl rustc_typeck::check::FnCtxt>::check_match
  56: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  57: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  58: rustc_typeck::check::FnCtxt::check_block_with_expected
  59: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  60: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  61: rustc_typeck::check::FnCtxt::check_block_with_expected
  62: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  63: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  64: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_return_expr
  65: rustc_typeck::check::check_fn
  66: rustc_typeck::check::closure::<impl rustc_typeck::check::FnCtxt>::check_expr_closure
  67: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  68: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  69: rustc_typeck::check::FnCtxt::check_argument_types
  70: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::confirm_builtin_call
  71: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::check_call
  72: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  73: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  74: rustc_typeck::check::FnCtxt::check_argument_types
  75: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::confirm_builtin_call
  76: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::check_call
  77: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  78: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  79: rustc_typeck::check::FnCtxt::check_block_with_expected
  80: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  81: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  82: rustc_typeck::check::FnCtxt::check_decl_initializer
  83: rustc_typeck::check::FnCtxt::check_decl_local
  84: rustc_typeck::check::FnCtxt::check_stmt
  85: rustc_typeck::check::FnCtxt::check_block_with_expected
  86: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  87: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  88: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_return_expr
  89: rustc_typeck::check::check_fn
  90: rustc::ty::context::tls::with_context::{{closure}}
  91: rustc_typeck::check::typeck_tables_of
  92: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
  93: rustc::dep_graph::graph::DepGraph::with_task_impl
  94: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query
  95: rustc::ty::query::plumbing::force_from_dep_node
  96: rustc::dep_graph::graph::DepGraph::try_mark_previous_green
  97: rustc::dep_graph::graph::DepGraph::try_mark_green
  98: rustc::dep_graph::graph::DepGraph::try_mark_green_and_read
  99: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
 100: rustc_typeck::collect::type_of
 101: rustc::ty::query::__query_compute::type_of
 102: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::type_of>::compute
 103: rustc::dep_graph::graph::DepGraph::with_task_impl
 104: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
 105: rustc_hir::intravisit::walk_expr
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.42.0-nightly (3291ae339 2020-01-15) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck_tables_of] processing `<shell::filesystem_shell::FilesystemShell as shell::shell::Shell>::ls`
#1 [typeck_tables_of] processing `<shell::filesystem_shell::FilesystemShell as shell::shell::Shell>::ls::{{closure}}#0`
#2 [type_of] processing `<shell::filesystem_shell::FilesystemShell as shell::shell::Shell>::ls::{{closure}}#0`
#3 [collect_mod_item_types] collecting item types in module `shell::filesystem_shell`
#4 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `nu`.
