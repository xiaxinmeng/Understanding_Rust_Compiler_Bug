
$ RUST_BACKTRACE=1 cargo build
   Compiling ice v0.1.0 (file:///home/hanno/tmp/Projects/rust-ice)
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.23.0-nightly (5041b3bb3 2017-11-19) running on x86_64-unknown-linux-gnu

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'assertion failed: !(self.has_self && idx == 0)', /checkout/src/librustc/ty/mod.rs:796:16
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at /checkout/src/libstd/sys_common/backtrace.rs:68
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/sys_common/backtrace.rs:57
             at /checkout/src/libstd/panicking.rs:381
   3: std::panicking::default_hook
             at /checkout/src/libstd/panicking.rs:391
   4: std::panicking::rust_panic_with_hook
             at /checkout/src/libstd/panicking.rs:577
   5: std::panicking::begin_panic
   6: rustc::ty::Generics::type_param
   7: rustc::infer::error_reporting::<impl rustc::infer::InferCtxt<'a, 'gcx, 'tcx>>::report_region_errors
   8: rustc::infer::InferCtxt::resolve_regions_and_report_errors
   9: rustc_typeck::check::regionck::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::regionck_item
  10: <std::thread::local::LocalKey<T>>::with
  11: rustc::ty::context::GlobalCtxt::enter_local
  12: _ZN12rustc_typeck5check7wfcheck26CheckTypeWellFormedVisitor21check_associated_item17h6a5d2e2fa0770148E.llvm.7C6DB860
  13: rustc::hir::Crate::visit_all_item_likes
  14: rustc::session::Session::track_errors
  15: rustc::util::common::time
  16: rustc_typeck::check_crate
  17: <std::thread::local::LocalKey<T>>::with
  18: <std::thread::local::LocalKey<T>>::with
  19: rustc::ty::context::TyCtxt::create_and_enter
  20: rustc_driver::driver::compile_input
  21: rustc_driver::run_compiler
