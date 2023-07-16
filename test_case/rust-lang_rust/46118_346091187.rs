
$ cd borrowck/nll
$ cargo rustc -- -Znll

   Compiling nll v0.1.0 (file:///home/ritiek/Downloads/borrowck/nll)
error: internal compiler error: /checkout/src/librustc_mir/transform/nll/mod.rs:174: region is not an ReVar: ReLateBound(DebruijnIndex { depth: 1 }, BrAnon(0))

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.23.0-nightly (5041b3bb3 2017-11-19) running on x86_64-unknown-linux-gnu

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc_errors/lib.rs:471:8
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
   6: rustc_errors::Handler::bug
   7: <std::thread::local::LocalKey<T>>::with
   8: rustc::ty::context::tls::with_opt
   9: _ZN5rustc7session16opt_span_bug_fmt17h45cda87bbef5d078E.llvm.3FD30DA7
  10: rustc::session::bug_fmt
  11: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_visit_with
  12: rustc::ty::fold::TypeFoldable::visit_with
  13: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_visit_with
  14: _ZN9rustc_mir9transform3nll21constraint_generation20ConstraintGeneration24add_liveness_constraints28_$u7b$$u7b$closure$u7d$$u7d$17haf91ba37f4fbd56eE.llvm.29036A65
  15: rustc_mir::transform::nll::compute_regions
  16: rustc_mir::borrow_check::do_mir_borrowck
  17: <std::thread::local::LocalKey<T>>::with
  18: rustc::ty::context::GlobalCtxt::enter_local
  19: _ZN9rustc_mir12borrow_check12mir_borrowck17hd2e1eca39abe2434E.llvm.20954609
  20: rustc::ty::maps::<impl rustc::ty::maps::queries::mir_borrowck<'tcx>>::compute_result
  21: _ZN5rustc9dep_graph5graph8DepGraph14with_task_impl17hdca1bda1b737f230E.llvm.7DE9159C
  22: rustc_errors::Handler::track_diagnostics
  23: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
  24: rustc::ty::maps::<impl rustc::ty::maps::queries::mir_borrowck<'tcx>>::force
  25: rustc::ty::maps::<impl rustc::ty::maps::queries::mir_borrowck<'tcx>>::try_get
  26: rustc::ty::maps::TyCtxtAt::mir_borrowck
  27: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::mir_borrowck
  28: _ZN12rustc_driver6driver27phase_3_run_analysis_passes28_$u7b$$u7b$closure$u7d$$u7d$28_$u7b$$u7b$closure$u7d$$u7d$17h9f2d943c7a15a43cE.llvm.ABDECBA6
  29: <std::thread::local::LocalKey<T>>::with
  30: <std::thread::local::LocalKey<T>>::with
  31: rustc::ty::context::TyCtxt::create_and_enter
  32: rustc_driver::driver::compile_input
  33: rustc_driver::run_compiler

error: Could not compile `nll`.

To learn more, run the command again with --verbose.
