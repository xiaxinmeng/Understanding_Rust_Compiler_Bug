
error: could not evaluate constant pattern
  --> test.rs:13:37
   |
13 |     matches!(GetTypeId::<T>::VALUE, GetTypeId::<T>::VALUE)
   |                                     ^^^^^^^^^^^^^^^^^^^^^

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', src/librustc_errors/lib.rs:942:13
stack backtrace:
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: std::io::Write::write_fmt
   3: std::panicking::default_hook::{{closure}}
   4: std::panicking::default_hook
   5: rustc_driver::report_ice
   6: std::panicking::rust_panic_with_hook
   7: std::panicking::begin_panic
   8: rustc_errors::HandlerInner::emit_diagnostic
   9: rustc_errors::Handler::emit_diag_at_span
  10: rustc_mir_build::hair::pattern::PatCtxt::lower_path
  11: rustc_mir_build::hair::pattern::PatCtxt::lower_pattern
  12: rustc_mir_build::hair::pattern::check_match::MatchVisitor::lower_pattern
  13: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::fold
  14: <rustc_mir_build::hair::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
  15: <rustc_mir_build::hair::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
  16: rustc_mir_build::hair::pattern::check_match::check_match
  17: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::check_match>::compute
  18: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  19: rustc_data_structures::stack::ensure_sufficient_stack
  20: rustc_query_system::query::plumbing::get_query_impl
  21: rustc_query_system::query::plumbing::ensure_query_impl
  22: rustc_session::utils::<impl rustc_session::session::Session>::time
  23: rustc_session::utils::<impl rustc_session::session::Session>::time
  24: rustc_interface::passes::analysis
  25: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
  26: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  27: rustc_data_structures::stack::ensure_sufficient_stack
  28: rustc_query_system::query::plumbing::get_query_impl
  29: rustc_middle::ty::context::tls::enter_global
  30: rustc_interface::interface::run_compiler_in_existing_thread_pool
  31: scoped_tls::ScopedKey<T>::set
  32: rustc_ast::attr::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.46.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z treat-err-as-bug
