
thread 'rustc' panicked at 'Box<Any>', /rustc/f9a3086363f214f2b56bef30f0ac572e1a9127f1\src\libstd\macros.rs:13:23
stack backtrace:
   0: std::backtrace_rs::backtrace::dbghelp::trace
             at /rustc/f9a3086363f214f2b56bef30f0ac572e1a9127f1\/src\libstd\..\backtrace\src\backtrace/dbghelp.rs:98
   1: std::backtrace_rs::backtrace::trace_unsynchronized
             at /rustc/f9a3086363f214f2b56bef30f0ac572e1a9127f1\/src\libstd\..\backtrace\src\backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at /rustc/f9a3086363f214f2b56bef30f0ac572e1a9127f1\/src\libstd\sys_common/backtrace.rs:77
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at /rustc/f9a3086363f214f2b56bef30f0ac572e1a9127f1\/src\libstd\sys_common/backtrace.rs:58
   4: core::fmt::write
             at /rustc/f9a3086363f214f2b56bef30f0ac572e1a9127f1\/src\libcore\fmt/mod.rs:1117
   5: std::io::Write::write_fmt
             at /rustc/f9a3086363f214f2b56bef30f0ac572e1a9127f1\/src\libstd\io/mod.rs:1508
   6: std::sys_common::backtrace::_print
             at /rustc/f9a3086363f214f2b56bef30f0ac572e1a9127f1\/src\libstd\sys_common/backtrace.rs:61
   7: std::sys_common::backtrace::print
             at /rustc/f9a3086363f214f2b56bef30f0ac572e1a9127f1\/src\libstd\sys_common/backtrace.rs:48
   8: std::panicking::default_hook::{{closure}}
             at /rustc/f9a3086363f214f2b56bef30f0ac572e1a9127f1\/src\libstd/panicking.rs:198
   9: std::panicking::default_hook
             at /rustc/f9a3086363f214f2b56bef30f0ac572e1a9127f1\/src\libstd/panicking.rs:217
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at /rustc/f9a3086363f214f2b56bef30f0ac572e1a9127f1\/src\libstd/panicking.rs:530
  12: std::panicking::begin_panic
  13: rustc_errors::HandlerInner::span_bug
  14: rustc_errors::Handler::span_bug
  15: rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}
  16: rustc_middle::ty::context::tls::with_opt::{{closure}}
  17: rustc_middle::ty::context::tls::with_opt
  18: rustc_middle::util::bug::opt_span_bug_fmt
  19: rustc_middle::util::bug::span_bug_fmt
  20: rustc_typeck::check::FnCtxt::local_ty::{{closure}}
  21: rustc_typeck::check::FnCtxt::local_ty
  22: rustc_typeck::check::FnCtxt::instantiate_value_path
  23: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  24: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation
  25: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  26: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation
  27: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  28: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation
  29: <smallvec::SmallVec<A> as core::iter::traits::collect::Extend<<A as smallvec::Array>::Item>>::extend
  30: <T as rustc_middle::ty::context::InternIteratorElement<T,R>>::intern_with
  31: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  32: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation
  33: rustc_typeck::check::_match::<impl rustc_typeck::check::FnCtxt>::check_match
  34: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  35: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation
  36: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  37: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation
  38: rustc_typeck::check::FnCtxt::check_argument_types
  39: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::confirm_builtin_call
  40: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::check_call
  41: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  42: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation
  43: rustc_typeck::check::FnCtxt::check_argument_types
  44: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::confirm_builtin_call
  45: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::check_call
  46: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  47: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation
  48: rustc_typeck::check::FnCtxt::check_stmt
  49: rustc_typeck::check::FnCtxt::check_block_with_expected
  50: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  51: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation
  52: rustc_typeck::check::_match::<impl rustc_typeck::check::FnCtxt>::check_match
  53: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  54: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation
  55: rustc_typeck::check::FnCtxt::check_stmt
  56: rustc_typeck::check::FnCtxt::check_block_with_expected
  57: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  58: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation
  59: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_return_expr
  60: rustc_typeck::check::check_fn
  61: rustc_infer::infer::InferCtxtBuilder::enter
  62: rustc_typeck::check::typeck
  63: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::typeck>::compute
  64: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  65: rustc_query_system::query::plumbing::get_query_impl
  66: rustc_query_system::query::plumbing::ensure_query_impl
  67: rustc_typeck::check::typeck_item_bodies
  68: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::typeck_item_bodies>::compute
  69: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  70: rustc_query_system::query::plumbing::get_query_impl
  71: rustc_typeck::check_crate
  72: rustc_interface::passes::analysis
  73: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
  74: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  75: rustc_data_structures::stack::ensure_sufficient_stack
  76: rustc_query_system::query::plumbing::get_query_impl
  77: rustc_middle::ty::context::tls::enter_global
  78: rustc_interface::interface::create_compiler_and_run
  79: scoped_tls::ScopedKey<T>::set
  80: rustc_ast::attr::with_session_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
