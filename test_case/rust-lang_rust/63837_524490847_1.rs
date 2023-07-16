
error[E0267]: `break` inside of a closure
 --> src/main.rs:4:42
  |
4 |         macro_rules! exit_loop { () => { break 'outer; } }
  |                                          ^^^^^^^^^^^^ cannot `break` inside of a closure
...
7 |             None.ok_or_else(||{exit_loop!();});
  |                             -- ------------- in this macro invocation
  |                             |
  |                             enclosing closure

error: internal compiler error: src/librustc_typeck/check/mod.rs:509: could not find enclosing breakable with id HirId { owner: DefIndex(12), local_id: 31 }

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:643:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.35/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.35/src/backtrace/mod.rs:66
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
  16: rustc_typeck::check::EnclosingBreakables::find_breakable::{{closure}}
  17: rustc_typeck::check::EnclosingBreakables::find_breakable
  18: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  19: rustc_typeck::check::FnCtxt::check_stmt
  20: rustc_typeck::check::FnCtxt::check_block_with_expected
  21: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  22: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_return_expr
  23: rustc_typeck::check::check_fn
  24: rustc_typeck::check::closure::<impl rustc_typeck::check::FnCtxt>::check_expr_closure
  25: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  26: rustc_typeck::check::FnCtxt::check_argument_types
  27: rustc_typeck::check::FnCtxt::check_method_argument_types
  28: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  29: rustc_typeck::check::FnCtxt::check_stmt
  30: rustc_typeck::check::FnCtxt::check_block_with_expected
  31: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  32: rustc_typeck::check::FnCtxt::check_block_with_expected
  33: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  34: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_meets_expectation_or_error
  35: rustc_typeck::check::FnCtxt::check_stmt
  36: rustc_typeck::check::FnCtxt::check_block_with_expected
  37: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  38: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_return_expr
  39: rustc_typeck::check::check_fn
  40: rustc::ty::context::GlobalCtxt::enter_local
  41: rustc_typeck::check::typeck_tables_of
  42: rustc::ty::query::__query_compute::typeck_tables_of
  43: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
  44: rustc::dep_graph::graph::DepGraph::with_task_impl
  45: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  46: rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners
  47: rustc_typeck::check::typeck_item_bodies
  48: rustc::ty::query::__query_compute::typeck_item_bodies
  49: rustc::dep_graph::graph::DepGraph::with_task_impl
  50: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  51: rustc::util::common::time
  52: rustc_typeck::check_crate
  53: rustc_interface::passes::analysis
  54: rustc::ty::query::__query_compute::analysis
  55: rustc::dep_graph::graph::DepGraph::with_task_impl
  56: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  57: rustc::ty::context::tls::enter_global
  58: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  59: rustc_interface::passes::create_global_ctxt::{{closure}}
  60: rustc_interface::passes::BoxedGlobalCtxt::enter
  61: rustc_interface::interface::run_compiler_in_existing_thread_pool
  62: std::thread::local::LocalKey<T>::with
  63: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
query stack during panic:
#0 [typeck_tables_of] processing `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0267`.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.39.0-nightly (760226733 2019-08-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `playground`.
