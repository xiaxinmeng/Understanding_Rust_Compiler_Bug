
thread 'rustc' panicked at 'assertion failed: !value.has_escaping_bound_vars()', src/librustc/ty/sty.rs:924:9
stack backtrace:
   0: backtrace::backtrace::trace_unsynchronized
   1: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   2: core::fmt::write
   3: std::io::Write::write_fmt
   4: std::panicking::take_hook
   5: std::panicking::take_hook
   6: clippy_driver::report_clippy_ice
   7: std::panicking::rust_panic_with_hook
   8: std::panicking::begin_panic
   9: rustc::traits::type_known_to_meet_bound_modulo_regions
  10: rustc::ty::context::GlobalCtxt::enter_local
  11: rustc::ty::util::is_freeze_raw
  12: rustc::ty::util::is_freeze_raw
  13: rustc::ty::query::__query_compute::is_freeze_raw
  14: <rustc::traits::Vtable<N> as core::clone::Clone>::clone
  15: rustc::dep_graph::graph::DepGraph::with_task_impl
  16: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  17: rustc::ty::util::<impl rustc::ty::TyS>::is_freeze
  18: <clippy_lints::mut_key::MutableKeyType as rustc::lint::LateLintPass>::check_local
  19: <clippy_lints::mut_key::MutableKeyType as rustc::lint::LateLintPass>::check_local
  20: <rustc::lint::context::LateLintPassObjects as rustc::lint::LateLintPass>::check_item
  21: rustc::hir::intravisit::Visitor::visit_nested_item
  22: rustc::hir::intravisit::walk_crate
  23: rustc::lint::context::late_lint_pass_crate
  24: rustc::lint::context::late_lint_crate
  25: rustc::util::common::time
  26: rustc::util::common::time
  27: std::panicking::begin_panic
  28: _rust_maybe_catch_panic
  29: std::panicking::try::do_call
  30: _rust_maybe_catch_panic
  31: std::panic::catch_unwind
  32: rustc_interface::passes::analysis::{{closure}}
  33: rustc_interface::passes::create_global_ctxt
  34: rustc::ty::query::__query_compute::analysis
  35: rustc::dep_graph::graph::DepGraph::with_task_impl
  36: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  37: rustc::ty::context::tls::enter_global
  38: rustc_interface::interface::run_compiler_in_existing_thread_pool
  39: std::thread::local::LocalKey<T>::with
  40: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.0.212 (0fec5905 2019-12-27)

query stack during panic:
#0 [is_freeze_raw] computing whether `&'a Bar` is freeze
#1 [analysis] running analysis passes on this crate
end of query stack
