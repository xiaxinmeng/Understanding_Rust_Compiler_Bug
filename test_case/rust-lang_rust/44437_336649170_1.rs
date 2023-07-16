
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/src/libcore/option.rs:335:20
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at /checkout/src/libstd/sys_common/backtrace.rs:69
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/sys_common/backtrace.rs:58
             at /checkout/src/libstd/panicking.rs:381
   3: std::panicking::default_hook
             at /checkout/src/libstd/panicking.rs:391
   4: std::panicking::rust_panic_with_hook
             at /checkout/src/libstd/panicking.rs:577
   5: std::panicking::begin_panic
             at /checkout/src/libstd/panicking.rs:538
   6: std::panicking::begin_panic_fmt
             at /checkout/src/libstd/panicking.rs:522
   7: rust_begin_unwind
             at /checkout/src/libstd/panicking.rs:498
   8: core::panicking::panic_fmt
             at /checkout/src/libcore/panicking.rs:71
   9: core::panicking::panic
             at /checkout/src/libcore/panicking.rs:51
  10: <rustc::ty::fold::BottomUpFolder<'a, 'gcx, 'tcx, F> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  11: rustc::ty::subst::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::Slice<rustc::ty::subst::Kind<'tcx>>>::super_fold_with
  12: <rustc::ty::fold::BottomUpFolder<'a, 'gcx, 'tcx, F> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  13: rustc_typeck::check::check_fn
  14: rustc_typeck::check::closure::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::check_closure
  15: rustc_typeck::check::FnCtxt::check_expr_kind
  16: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref
  17: rustc_typeck::check::FnCtxt::check_argument_types
  18: rustc_typeck::check::FnCtxt::check_method_argument_types
  19: rustc_typeck::check::FnCtxt::check_expr_kind
  20: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref
  21: rustc_typeck::check::FnCtxt::check_block_with_expected::{{closure}}
  22: rustc_typeck::check::FnCtxt::check_block_with_expected
  23: rustc_typeck::check::FnCtxt::check_expr_kind
  24: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref
  25: rustc_typeck::check::FnCtxt::check_return_expr
  26: rustc_typeck::check::check_fn
  27: rustc_typeck::check::typeck_tables_of::{{closure}}
  28: rustc_typeck::check::typeck_tables_of
  29: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::force
  30: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::try_get
  31: rustc::ty::maps::TyCtxtAt::typeck_tables_of
  32: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_tables_of
  33: rustc_typeck::check::typeck_item_bodies
  34: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::force
  35: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::try_get
  36: rustc::ty::maps::TyCtxtAt::typeck_item_bodies
  37: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_item_bodies
  38: rustc_typeck::check_crate
  39: rustc::ty::context::TyCtxt::create_and_enter
  40: rustc_driver::driver::compile_input
  41: rustc_driver::run_compiler
