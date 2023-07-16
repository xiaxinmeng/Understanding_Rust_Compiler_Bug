
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'index out of bounds: the len is 5 but the index is 7', ../src/libcollections/vec.rs:1166
stack backtrace:
   1:        0x1117c7f38 - sys::backtrace::tracing::imp::write::h83bc1946ac132a7d0Gu
   2:        0x1117d1475 - panicking::default_hook::_$u7b$$u7b$closure$u7d$$u7d$::closure.43767
   3:        0x1117d0fd7 - panicking::default_hook::had684d056a84b05cPgz
   4:        0x111798336 - sys_common::unwind::begin_unwind_inner::hfef527c9f2434064HGt
   5:        0x111799f4e - sys_common::unwind::begin_unwind_fmt::h02d40e945b8e457cNFt
   6:        0x1117c5807 - rust_begin_unwind
   7:        0x11180a6d0 - panicking::panic_fmt::h54d7b44acd72bb46zML
   8:        0x11180a8e0 - panicking::panic_bounds_check::hb99e0b711696a86cFLL
   9:        0x10e1227c0 - middle::infer::InferCtxt<'a, 'tcx>::shallow_resolve::h43c92678dc539950AtD
  10:        0x10e1fbc7b - middle::traits::fulfill::process_predicate::hc70860709ec86ab05US
  11:        0x10e1faa83 - obligation_forest::ObligationForest<O, T>::process_obligations::h18416070063450621727
  12:        0x10e1f619c - middle::traits::fulfill::FulfillmentContext<'tcx>::select_where_possible::h817133e3c0e6314baNS
  13:        0x10dc9b354 - check::FnCtxt<'a, 'tcx>::select_obligations_where_possible::h7ea8359c24588fcdiMq
  14:        0x10dcaeccc - check::FnCtxt<'a, 'tcx>::resolve_type_vars_if_possible::hfde6a734ee0b83c8GSp
  15:        0x10dcc570a - check::demand::coerce::h59fab50398c45a417Cf
  16:        0x10dcc30e6 - check::check_argument_types::h7425e24ea32ed15f6ar
  17:        0x10dd3d999 - check::callee::confirm_builtin_call::h47e6b2f48a31900cfkm
  18:        0x10dcdc120 - check::callee::check_call::h0c0b2518a65d933bkbm
  19:        0x10dcc69ca - check::check_expr_with_expectation_and_lvalue_pref::he02550a8ac3fc525bJr
  20:        0x10dcf6b52 - check::check_stmt::hfd7db92334e1e068IYs
  21:        0x10dca193a - check::check_block_with_expected::h29cb83403d5adabbv2s
  22:        0x10dc98348 - check::check_fn::hf8f01d7e0d5b7dcayWo
  23:        0x10dc95a2f - check::check_bare_fn::hf7c744d6fc7c32be1Lo
  24:        0x10dc908d9 - check::check_item_body::h2d3043343fe0fdd1Dbp
  25:        0x10dc8878b - check::check_item_bodies::h457431c5d3d2ced0OJo
  26:        0x10dc80005 - check_crate::h3e3d779ef330fadcCFC
  27:        0x10d323db1 - driver::phase_3_run_analysis_passes::_$u7b$$u7b$closure$u7d$$u7d$::closure.30095
  28:        0x10d3223e9 - middle::ty::context::TyCtxt<'tcx>::create_and_enter::h11812805895441873020
  29:        0x10d31edad - driver::phase_3_run_analysis_passes::h5261001332275006052
  30:        0x10d2f1df7 - driver::compile_input::he5def18759d9424cPca
  31:        0x10d2df6f6 - run_compiler::h61ff55f691b932efVMc
  32:        0x10d2dcca2 - sys_common::unwind::try::try_fn::h2984156204492186173
  33:        0x1117c579b - __rust_try
  34:        0x1117c5723 - sys_common::unwind::inner_try::hbef67b929e699248JDt
  35:        0x10d2dd539 - boxed::F.FnBox<A>::call_box::h8793623295134616532
  36:        0x1117d02c8 - sys::thread::Thread::new::thread_start::h68477ad6a3ba8036fvy
  37:     0x7fff89eb999c - _pthread_body
  38:     0x7fff89eb9919 - _pthread_start
