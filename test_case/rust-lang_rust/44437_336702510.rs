

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.22.0-dev running on x86_64-unknown-linux-gnu

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', source://rust/src/libcore/option.rs:335:20
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at source://rust/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at source://rust/src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at source://rust/src/libstd/sys_common/backtrace.rs:60
             at source://rust/src/libstd/panicking.rs:381
   3: std::panicking::default_hook
             at source://rust/src/libstd/panicking.rs:391
   4: std::panicking::rust_panic_with_hook
             at source://rust/src/libstd/panicking.rs:577
   5: std::panicking::begin_panic
             at source://rust/src/libstd/panicking.rs:538
   6: std::panicking::begin_panic_fmt
             at source://rust/src/libstd/panicking.rs:522
   7: rust_begin_unwind
             at source://rust/src/libstd/panicking.rs:498
   8: core::panicking::panic_fmt
             at source://rust/src/libcore/panicking.rs:71
   9: core::panicking::panic
             at source://rust/src/libcore/panicking.rs:51
  10: <rustc::ty::fold::BottomUpFolder<'a, 'gcx, 'tcx, F> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
             at source://rust/src/libcore/macros.rs:20
             at source://rust/src/librustc_typeck/check/mod.rs:1925
             at source://rust/src/librustc/ty/fold.rs:184
  11: rustc::ty::subst::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::Slice<rustc::ty::subst::Kind<'tcx>>>::super_fold_with
             at source://rust/src/librustc/ty/structural_impls.rs:690
             at source://rust/src/librustc/ty/subst.rs:113
             at source://rust/src/librustc/ty/fold.rs:53
             at source://rust/src/librustc/ty/subst.rs:312
             at source://rust/src/libcore/ops/function.rs:271
             at source://rust/src/libcore/option.rs:398
             at source://rust/src/libcore/iter/mod.rs:1123
             at source://rust/src/librustc_data_structures/array_vec.rs:197
             at source://rust/src/librustc_data_structures/accumulate_vec.rs:114
             at source://rust/src/libcore/iter/iterator.rs:1298
             at source://rust/src/librustc/ty/subst.rs:312
  12: <rustc::ty::fold::BottomUpFolder<'a, 'gcx, 'tcx, F> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
             at source://rust/src/librustc/ty/fold.rs:53
             at source://rust/src/librustc/ty/structural_impls.rs:660
             at source://rust/src/librustc/ty/fold.rs:183
  13: rustc_typeck::check::check_fn
             at source://rust/src/librustc/ty/structural_impls.rs:690
             at source://rust/src/librustc_typeck/check/mod.rs:1921
             at source://rust/src/librustc_typeck/check/mod.rs:1019
  14: rustc_typeck::check::closure::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::check_closure
             at source://rust/src/librustc_typeck/check/closure.rs:84
  15: rustc_typeck::check::FnCtxt::check_expr_kind
             at source://rust/src/librustc_typeck/check/closure.rs:44
             at source://rust/src/librustc_typeck/check/mod.rs:3836
  16: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref
             at source://rust/src/librustc_typeck/check/mod.rs:3465
  17: rustc_typeck::check::FnCtxt::check_argument_types
             at source://rust/src/librustc_typeck/check/mod.rs:2745
             at source://rust/src/librustc_typeck/check/mod.rs:2573
  18: rustc_typeck::check::FnCtxt::check_method_argument_types
             at source://rust/src/librustc_typeck/check/mod.rs:2384
  19: rustc_typeck::check::FnCtxt::check_expr_kind
             at source://rust/src/librustc_typeck/check/mod.rs:2866
             at source://rust/src/librustc_typeck/check/mod.rs:3845
  20: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref
             at source://rust/src/librustc_typeck/check/mod.rs:3465
  21: rustc_typeck::check::FnCtxt::check_block_with_expected::{{closure}}
             at source://rust/src/librustc_typeck/check/mod.rs:2745
             at source://rust/src/librustc_typeck/check/mod.rs:2749
             at source://rust/src/librustc_typeck/check/mod.rs:4208
             at source://rust/src/librustc_typeck/check/mod.rs:4272
  22: rustc_typeck::check::FnCtxt::check_block_with_expected
             at source://rust/src/librustc_typeck/check/mod.rs:4950
             at source://rust/src/librustc_typeck/check/mod.rs:4270
  23: rustc_typeck::check::FnCtxt::check_expr_kind
             at source://rust/src/librustc_typeck/check/mod.rs:3839
  24: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref
             at source://rust/src/librustc_typeck/check/mod.rs:3465
  25: rustc_typeck::check::FnCtxt::check_return_expr
             at source://rust/src/librustc_typeck/check/mod.rs:2745
             at source://rust/src/librustc_typeck/check/mod.rs:2739
             at source://rust/src/librustc_typeck/check/mod.rs:2880
  26: rustc_typeck::check::check_fn
             at source://rust/src/librustc_typeck/check/mod.rs:1074
  27: rustc_typeck::check::typeck_tables_of::{{closure}}
             at source://rust/src/librustc_typeck/check/mod.rs:882
  28: rustc_typeck::check::typeck_tables_of
             at source://rust/src/librustc_typeck/check/mod.rs:598
             at source://rust/src/librustc/infer/mod.rs:375
             at source://rust/src/librustc/ty/context.rs:1477
             at source://rust/src/libstd/thread/local.rs:379
             at source://rust/src/libstd/thread/local.rs:293
             at source://rust/src/librustc/ty/context.rs:1474
             at source://rust/src/librustc/ty/context.rs:1285
             at source://rust/src/librustc/infer/mod.rs:375
             at source://rust/src/librustc_typeck/check/mod.rs:598
             at source://rust/src/librustc_typeck/check/mod.rs:866
  29: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::force
             at source://rust/src/librustc/ty/maps/plumbing.rs:349
             at source://rust/src/librustc/dep_graph/graph.rs:272
             at source://rust/src/librustc/ty/maps/plumbing.rs:395
             at source://rust/src/librustc_errors/lib.rs:555
             at source://rust/src/librustc/ty/maps/plumbing.rs:394
             at source://rust/src/librustc/ty/maps/plumbing.rs:130
             at source://rust/src/librustc/ty/maps/plumbing.rs:393
  30: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::try_get
             at source://rust/src/librustc/ty/maps/plumbing.rs:279
             at source://rust/src/librustc/ty/maps/plumbing.rs:423
  31: rustc::ty::maps::TyCtxtAt::typeck_tables_of
             at source://rust/src/librustc/ty/maps/plumbing.rs:462
  32: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_tables_of
             at source://rust/src/librustc/ty/maps/plumbing.rs:455
  33: rustc_typeck::check::typeck_item_bodies
             at source://rust/src/librustc_typeck/check/mod.rs:731
             at source://rust/src/librustc/session/mod.rs:268
             at source://rust/src/librustc_typeck/check/mod.rs:729
  34: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::force
             at source://rust/src/librustc/ty/maps/plumbing.rs:349
             at source://rust/src/librustc/dep_graph/graph.rs:272
             at source://rust/src/librustc/ty/maps/plumbing.rs:395
             at source://rust/src/librustc_errors/lib.rs:555
             at source://rust/src/librustc/ty/maps/plumbing.rs:394
             at source://rust/src/librustc/ty/maps/plumbing.rs:130
             at source://rust/src/librustc/ty/maps/plumbing.rs:393
  35: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::try_get
             at source://rust/src/librustc/ty/maps/plumbing.rs:279
             at source://rust/src/librustc/ty/maps/plumbing.rs:423
  36: rustc::ty::maps::TyCtxtAt::typeck_item_bodies
             at source://rust/src/librustc/ty/maps/plumbing.rs:462
  37: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_item_bodies
             at source://rust/src/librustc/ty/maps/plumbing.rs:455
  38: rustc_typeck::check_crate
             at source://rust/src/librustc_typeck/check/mod.rs:722
             at source://rust/src/librustc_typeck/lib.rs:322
             at source://rust/src/librustc/util/common.rs:120
             at source://rust/src/librustc_typeck/lib.rs:322
  39: rustc::ty::context::TyCtxt::create_and_enter
             at source://rust/src/librustc_driver/driver.rs:1068
             at source://rust/src/librustc/ty/context.rs:1477
             at source://rust/src/libstd/thread/local.rs:379
             at source://rust/src/libstd/thread/local.rs:293
             at source://rust/src/librustc/ty/context.rs:1474
             at source://rust/src/librustc/ty/context.rs:1461
             at source://rust/src/libstd/thread/local.rs:379
             at source://rust/src/libstd/thread/local.rs:293
             at source://rust/src/librustc/ty/context.rs:1458
             at source://rust/src/librustc/ty/context.rs:1118
  40: rustc_driver::driver::compile_input
             at source://rust/src/librustc_driver/driver.rs:1045
             at source://rust/src/librustc_driver/driver.rs:211
  41: rustc_driver::run_compiler
             at source://rust/src/librustc_driver/lib.rs:250
