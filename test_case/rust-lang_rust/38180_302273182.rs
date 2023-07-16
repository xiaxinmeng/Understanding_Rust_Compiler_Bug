
error: internal compiler error: src/librustc_const_eval/_match.rs:246: bad constructor ConstantValue(Struct({num(89): Integral(U8(0))})) for adt Thing

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:473
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
...
  13: rustc_const_eval::_match::Constructor::variant_index_for_adt
             at /rust/src/librustc_const_eval/_match.rs:246
  14: rustc_const_eval::_match::constructor_sub_pattern_tys
             at /rust/src/librustc_const_eval/_match.rs:777
  15: rustc_const_eval::_match::is_useful_specialized
             at /rust/src/librustc_const_eval/_match.rs:673
  16: core::ops::impls::<impl core::ops::FnOnce<A> for &'a mut F>::call_once
             at /rust/src/librustc_const_eval/_match.rs:585
             at /rust/src/libcore/ops.rs:2711
  17: <&'a mut I as core::iter::iterator::Iterator>::next
             at /rust/src/libcore/option.rs:392
             at /rust/src/libcore/iter/mod.rs:1011
             at /rust/src/libcore/iter/iterator.rs:2202
  18: rustc_const_eval::_match::is_useful
             at /rust/src/libcore/iter/iterator.rs:1489
             at /rust/src/librustc_const_eval/_match.rs:584
  19: rustc_const_eval::_match::is_useful_specialized
             at /rust/src/librustc_const_eval/_match.rs:686
  20: core::ops::impls::<impl core::ops::FnOnce<A> for &'a mut F>::call_once
             at /rust/src/librustc_const_eval/_match.rs:585
             at /rust/src/libcore/ops.rs:2711
  21: <&'a mut I as core::iter::iterator::Iterator>::next
             at /rust/src/libcore/option.rs:392
             at /rust/src/libcore/iter/mod.rs:1011
             at /rust/src/libcore/iter/iterator.rs:2202
  22: rustc_const_eval::_match::is_useful
             at /rust/src/libcore/iter/iterator.rs:1489
             at /rust/src/librustc_const_eval/_match.rs:584
  23: rustc_const_eval::check_match::check_arms
             at /rust/src/librustc_const_eval/check_match.rs:316
  24: rustc_const_eval::_match::MatchCheckCtxt::create_and_enter
             at /rust/src/librustc_const_eval/check_match.rs:180
             at /rust/src/librustc_const_eval/_match.rs:168
  25: rustc_const_eval::check_match::MatchVisitor::check_match
             at /rust/src/librustc_const_eval/check_match.rs:158
  26: <rustc_const_eval::check_match::MatchVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_expr
             at /rust/src/librustc_const_eval/check_match.rs:82
  27: <rustc_const_eval::check_match::MatchVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_body
             at /rust/src/librustc_const_eval/check_match.rs:102
  28: <rustc_const_eval::check_match::OuterVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_fn
             at /rust/src/librustc_const_eval/check_match.rs:51
  29: rustc::hir::intravisit::walk_item
             at /rust/src/librustc/hir/intravisit.rs:457
  30: rustc::hir::Crate::visit_all_item_likes
             at /rust/src/librustc/hir/mod.rs:512
  31: rustc_const_eval::check_match::check_crate
             at /rust/src/librustc_const_eval/check_match.rs:61
  32: rustc::util::common::time
             at /rust/src/librustc_driver/driver.rs:992
             at /rust/src/librustc/util/common.rs:48
  33: <std::thread::local::LocalKey<T>>::with
             at /rust/src/librustc_driver/driver.rs:990
             at /rust/src/librustc/ty/context.rs:915
             at /rust/src/libstd/thread/local.rs:265
  34: rustc::ty::context::tls::enter
             at /rust/src/librustc/ty/context.rs:912
  35: <std::thread::local::LocalKey<T>>::with
             at /rust/src/librustc/ty/context.rs:899
             at /rust/src/libstd/thread/local.rs:265
  36: rustc::ty::context::tls::enter_global
             at /rust/src/librustc/ty/context.rs:896
  37: rustc::ty::context::TyCtxt::create_and_enter
             at /rust/src/librustc/ty/context.rs:706
  38: rustc_driver::driver::phase_3_run_analysis_passes
             at /rust/src/librustc_driver/driver.rs:942
  39: rustc_driver::driver::compile_input
             at /rust/src/librustc_driver/driver.rs:172
  40: rustc_driver::run_compiler
             at /rust/src/librustc_driver/lib.rs:224
