none
error: internal compiler error: src/librustc_passes/dead.rs:111: no type-dependent def for method

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:892:9
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
             at src/libstd/panicking.rs:195
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:215
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:467
  12: std::panicking::begin_panic
  13: rustc_errors::HandlerInner::bug
  14: rustc_errors::Handler::bug
  15: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  16: rustc::ty::context::tls::with_opt::{{closure}}
  17: rustc::ty::context::tls::with_opt
  18: rustc::util::bug::opt_span_bug_fmt
  19: rustc::util::bug::bug_fmt
  20: <rustc_passes::dead::MarkSymbolVisitor as rustc::hir::intravisit::Visitor>::visit_expr
  21: rustc::hir::intravisit::walk_expr
  22: rustc::hir::intravisit::walk_fn
  23: rustc::hir::intravisit::walk_item
  24: rustc_passes::dead::check_crate
  25: rustc_session::utils::<impl rustc_session::session::Session>::time
  26: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:79
  27: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  28: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:79
  29: rustc_session::utils::<impl rustc_session::session::Session>::time
  30: rustc_interface::passes::analysis
  31: rustc::ty::query::__query_compute::analysis
  32: rustc::dep_graph::graph::DepGraph::with_task_impl
  33: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  34: rustc::ty::context::tls::enter_global
  35: rustc_interface::interface::run_compiler_in_existing_thread_pool
  36: scoped_tls::ScopedKey<T>::set
  37: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.42.0-nightly (859764425 2020-01-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error
