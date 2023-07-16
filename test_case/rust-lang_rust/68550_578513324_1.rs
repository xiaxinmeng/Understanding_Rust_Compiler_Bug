
   Compiling playground v0.0.1 (/playground)
error: internal compiler error: src/librustc_mir/borrow_check/universal_regions.rs:729: cannot convert `ReEmpty` to a region vid

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:883:9
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
             at src/libcore/fmt/mod.rs:1052
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
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:476
  12: std::panicking::begin_panic
  13: rustc_errors::HandlerInner::bug
  14: rustc_errors::Handler::bug
  15: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  16: rustc::ty::context::tls::with_opt::{{closure}}
  17: rustc::ty::context::tls::with_opt
  18: rustc::util::bug::opt_span_bug_fmt
  19: rustc::util::bug::bug_fmt
  20: rustc_mir::borrow_check::universal_regions::UniversalRegionIndices::to_region_vid::{{closure}}
  21: rustc_mir::borrow_check::universal_regions::UniversalRegionIndices::to_region_vid
  22: rustc_mir::borrow_check::region_infer::RegionInferenceContext::eval_verify_bound
  23: rustc_mir::borrow_check::region_infer::RegionInferenceContext::eval_verify_bound
  24: rustc_mir::borrow_check::nll::compute_regions
  25: rustc_mir::borrow_check::do_mir_borrowck
  26: rustc::ty::context::GlobalCtxt::enter_local
  27: rustc_mir::borrow_check::mir_borrowck
  28: rustc::ty::query::__query_compute::mir_borrowck
  29: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::mir_borrowck>::compute
  30: rustc::dep_graph::graph::DepGraph::with_task_impl
  31: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  32: rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners
  33: rustc_session::utils::<impl rustc_session::session::Session>::time
  34: rustc_interface::passes::analysis
  35: rustc::ty::query::__query_compute::analysis
  36: rustc::dep_graph::graph::DepGraph::with_task_impl
  37: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  38: rustc::ty::context::tls::enter_global
  39: rustc_interface::interface::run_compiler_in_existing_thread_pool
  40: scoped_tls::ScopedKey<T>::set
  41: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.42.0-nightly (6d3f4e0aa 2020-01-25) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [mir_borrowck] processing `run`
#1 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error

error: could not compile `playground`.

To learn more, run the command again with --verbose.
