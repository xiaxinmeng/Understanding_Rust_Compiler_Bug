
Standard Error

   Compiling playground v0.0.1 (/playground)
error: internal compiler error: src/librustc_typeck/check/callee.rs:413: C-variadic functions are only valid with one or more fixed arguments
 --> src/main.rs:6:14
  |
6 |     unsafe { foo(1, 2, 3); }
  |              ^^^^^^^^^^^^

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:579:9
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
   9: rustc_errors::Handler::span_bug
  10: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  11: rustc::ty::context::tls::with_opt::{{closure}}
  12: rustc::ty::context::tls::with_context_opt
  13: rustc::ty::context::tls::with_opt
  14: rustc::util::bug::opt_span_bug_fmt
  15: rustc::util::bug::span_bug_fmt
  16: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::confirm_builtin_call
  17: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::check_call
  18: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  19: rustc_typeck::check::FnCtxt::check_stmt
  20: rustc_typeck::check::FnCtxt::check_block_with_expected
  21: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  22: rustc_typeck::check::FnCtxt::check_block_with_expected
  23: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  24: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_return_expr
  25: rustc_typeck::check::check_fn
  26: rustc::ty::context::GlobalCtxt::enter_local
  27: rustc_typeck::check::typeck_tables_of
  28: rustc::ty::query::__query_compute::typeck_tables_of
  29: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
  30: rustc::dep_graph::graph::DepGraph::with_task_impl
  31: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  32: rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners
  33: rustc_typeck::check::typeck_item_bodies
  34: rustc::ty::query::__query_compute::typeck_item_bodies
  35: rustc::dep_graph::graph::DepGraph::with_task_impl
  36: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  37: rustc::util::common::time
  38: rustc_typeck::check_crate
  39: rustc_interface::passes::analysis
  40: rustc::ty::query::__query_compute::analysis
  41: rustc::dep_graph::graph::DepGraph::with_task_impl
  42: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  43: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  44: rustc_interface::passes::create_global_ctxt::{{closure}}
  45: rustc_interface::interface::run_compiler_in_existing_thread_pool
  46: std::thread::local::LocalKey<T>::with
  47: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
query stack during panic:
#0 [typeck_tables_of] processing `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.38.0-nightly (534b42394 2019-08-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin
