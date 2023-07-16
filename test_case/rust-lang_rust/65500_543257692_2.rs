
error[E0603]: module `transaction` is private
 --> src/main.rs:1:15
  |
1 | use tiberius::transaction::Transaction;
  |               ^^^^^^^^^^^

thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/libcore/option.rs:378:21
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
  12: std::panicking::continue_panic_fmt
             at src/libstd/panicking.rs:376
  13: rust_begin_unwind
             at src/libstd/panicking.rs:303
  14: core::panicking::panic_fmt
             at src/libcore/panicking.rs:84
  15: core::panicking::panic
             at src/libcore/panicking.rs:49
  16: rustc_typeck::check::closure::<impl rustc_typeck::check::FnCtxt>::supplied_sig_of_closure
  17: rustc_typeck::check::closure::<impl rustc_typeck::check::FnCtxt>::check_expr_closure
  18: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  19: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  20: rustc_typeck::check::FnCtxt::check_argument_types
  21: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::confirm_builtin_call
  22: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::check_call
  23: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  24: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  25: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_return_expr
  26: rustc_typeck::check::check_fn
  27: rustc::ty::context::GlobalCtxt::enter_local
  28: rustc_typeck::check::typeck_tables_of
  29: rustc::ty::query::__query_compute::typeck_tables_of
  30: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
  31: rustc::dep_graph::graph::DepGraph::with_task_impl
  32: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  33: rustc::ty::query::__query_compute::typeck_tables_of
  34: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
  35: rustc::dep_graph::graph::DepGraph::with_task_impl
  36: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  37: rustc_typeck::collect::checked_type_of
  38: rustc_typeck::collect::type_of
  39: rustc::ty::query::__query_compute::type_of
  40: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::type_of>::compute
  41: rustc::dep_graph::graph::DepGraph::with_task_impl
  42: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  43: rustc::hir::intravisit::walk_expr
  44: rustc::hir::intravisit::Visitor::visit_fn
  45: rustc::hir::intravisit::walk_item
  46: <rustc_typeck::collect::CollectItemTypesVisitor as rustc::hir::intravisit::Visitor>::visit_item
  47: rustc::hir::map::Map::visit_item_likes_in_module
  48: rustc_typeck::collect::collect_mod_item_types
  49: rustc::ty::query::__query_compute::collect_mod_item_types
  50: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::collect_mod_item_types>::compute
  51: rustc::dep_graph::graph::DepGraph::with_task_impl
  52: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  53: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::ensure_query
  54: rustc_typeck::check_crate::{{closure}}::{{closure}}
  55: rustc::util::common::time
  56: rustc_typeck::check_crate
  57: rustc_interface::passes::analysis
  58: rustc::ty::query::__query_compute::analysis
  59: rustc::dep_graph::graph::DepGraph::with_task_impl
  60: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  61: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  62: rustc_interface::passes::create_global_ctxt::{{closure}}
  63: rustc_interface::passes::BoxedGlobalCtxt::enter
  64: rustc_interface::interface::run_compiler_in_existing_thread_pool
  65: std::thread::local::LocalKey<T>::with
  66: scoped_tls::ScopedKey<T>::set
  67: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.40.0-nightly (e413dc36a 2019-10-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck_tables_of] processing `suivi_log`
#1 [typeck_tables_of] processing `suivi_log::{{closure}}#0`
#2 [type_of] processing `suivi_log::{{closure}}#0`
#3 [collect_mod_item_types] collecting item types in top-level module
#4 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error

For more information about this error, try `rustc --explain E0603`.
error: could not compile `rs-suivi`.

To learn more, run the command again with --verbose.
