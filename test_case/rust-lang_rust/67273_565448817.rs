
$ RUST_BACKTRACE=1 rustc +gamma-stage1 foo.rs -Z no-codegen -Z report-delayed-bugs -Z treat-err-as-bug=1
error: internal compiler error[E0308]: match arms have incompatible types
 --> foo.rs:7:14
  |
4 | /     match i {
5 | |         1 => true,
  | |              ---- this is found to be of type `bool`
6 | |         2 => true,
  | |              ---- this is found to be of type `bool`
7 | |         3 => i = 1,
  | |              ^^^^^ expected `bool`, found `()`
8 | |         _ => (),
9 | |     }
  | |_____- `match` arms have incompatible types

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', src/librustc_errors/lib.rs:921:13stack backtrace:
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: std::io::Write::write_fmt
   3: std::panicking::default_hook::{{closure}}
   4: std::panicking::default_hook
   5: rustc_driver::report_ice
   6: std::panicking::rust_panic_with_hook
   7: std::panicking::begin_panic
   8: rustc_errors::HandlerInner::emit_diagnostic
   9: rustc_errors::HandlerInner::delay_as_bug
  10: rustc_errors::Handler::delay_as_bug
  11: rustc_errors::diagnostic_builder::DiagnosticBuilder::delay_as_bug
  12: rustc_typeck::check::coercion::CoerceMany<E>::coerce_inner
  13: rustc_typeck::check::_match::<impl rustc_typeck::check::FnCtxt>::check_match
  14: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  15: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  16: rustc_typeck::check::FnCtxt::with_breakable_ctxt
  17: rustc_typeck::check::FnCtxt::check_block_with_expected
  18: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  19: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  20: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_return_expr
  21: rustc_typeck::check::check_fn
  22: rustc::ty::context::tls::with_context::{{closure}}
  23: rustc::ty::context::GlobalCtxt::enter_local
  24: rustc_typeck::check::typeck_tables_of
  25: rustc::ty::query::__query_compute::typeck_tables_of
  26: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
  27: rustc::dep_graph::graph::DepGraph::with_task_impl
  28: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  29: rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners
  30: rustc_typeck::check::typeck_item_bodies
  31: rustc::ty::query::__query_compute::typeck_item_bodies
  32: rustc::dep_graph::graph::DepGraph::with_task_impl
  33: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  34: rustc::util::common::time
  35: rustc_typeck::check_crate
  36: rustc_interface::passes::analysis
  37: rustc::ty::query::__query_compute::analysis
  38: rustc::dep_graph::graph::DepGraph::with_task_impl
  39: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  40: rustc::ty::context::tls::enter_global
  41: rustc_interface::interface::run_compiler_in_existing_thread_pool
  42: std::thread::local::LocalKey<T>::with
  43: scoped_tls::ScopedKey<T>::set
  44: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.41.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z no-codegen -Z report-delayed-bugs -Z treat-err-as-bug=1

query stack during panic:
#0 [typeck_tables_of] processing `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack
