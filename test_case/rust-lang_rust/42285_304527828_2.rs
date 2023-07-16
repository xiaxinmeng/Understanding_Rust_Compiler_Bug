
rustc 1.19.0-nightly (28fd1e519 2017-05-27)
error: internal compiler error: /checkout/src/librustc_const_eval/_match.rs:246: bad constructor ConstantValue(Struct({})) for adt S

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc_errors/lib.rs:473
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at /checkout/src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/sys_common/backtrace.rs:60
             at /checkout/src/libstd/panicking.rs:355
   3: std::panicking::default_hook
             at /checkout/src/libstd/panicking.rs:365
   4: std::panicking::rust_panic_with_hook
             at /checkout/src/libstd/panicking.rs:549
   5: std::panicking::begin_panic
   6: rustc_errors::Handler::bug
   7: rustc::session::opt_span_bug_fmt::{{closure}}
   8: rustc::session::opt_span_bug_fmt
   9: rustc::session::bug_fmt
  10: rustc_const_eval::_match::Constructor::variant_index_for_adt
  11: rustc_const_eval::_match::constructor_sub_pattern_tys
  12: rustc_const_eval::_match::is_useful_specialized
  13: rustc_const_eval::_match::is_useful::{{closure}}
  14: <core::option::Option<T>>::map
  15: rustc_const_eval::_match::is_useful
  16: rustc_const_eval::check_match::MatchVisitor::check_match::{{closure}}
  17: <rustc_const_eval::check_match::MatchVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_expr
  18: <rustc_const_eval::check_match::MatchVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_expr
  19: <rustc_const_eval::check_match::OuterVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_fn
  20: rustc::hir::intravisit::walk_item
  21: rustc_const_eval::check_match::check_crate
  22: rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}
  23: rustc_driver::driver::phase_3_run_analysis_passes
  24: rustc_driver::driver::compile_input
  25: rustc_driver::run_compiler
