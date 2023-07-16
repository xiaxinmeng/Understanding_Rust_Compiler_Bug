
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:207
   3: std::panicking::default_hook
             at libstd/panicking.rs:223
   4: core::ops::function::Fn::call
             at librustc/util/common.rs:50
             at libcore/ops/function.rs:73
   5: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:403
   6: std::panicking::begin_panic
             at libstd/panicking.rs:365
   7: rustc_errors::Handler::span_bug
             at <panic macros>:3
   8: rustc::session::opt_span_bug_fmt::{{closure}}
             at librustc/session/mod.rs:1290
   9: <std::thread::local::LocalKey<T>>::try_with
             at librustc/ty/context.rs:1708
             at librustc/ty/context.rs:1697
             at libstd/thread/local.rs:290
  10: <std::thread::local::LocalKey<T>>::with
             at libstd/thread/local.rs:244
  11: rustc::ty::context::tls::with_opt
             at librustc/ty/context.rs:1693
             at librustc/ty/context.rs:1708
  12: rustc::session::opt_span_bug_fmt
             at librustc/session/mod.rs:1287
  13: rustc::session::span_bug_fmt
             at librustc/session/mod.rs:1278
  14: <rustc::ty::subst::SubstFolder<'a, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
             at librustc/ty/subst.rs:481
             at librustc/ty/subst.rs:456
  15: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with
             at librustc/ty/structural_impls.rs:1019
             at librustc/ty/structural_impls.rs:1054
             at librustc/ty/fold.rs:55
             at librustc/ty/structural_impls.rs:997
  16: <rustc::ty::subst::SubstFolder<'a, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
             at librustc/ty/subst.rs:459
  17: <rustc_data_structures::array_vec::ArrayVec<A> as core::iter::traits::Extend<<A as rustc_data_structures::array_vec::Array>::Element>>::extend
             at librustc/ty/structural_impls.rs:1019
             at librustc/ty/subst.rs:120
             at librustc/ty/fold.rs:55
             at librustc/ty/subst.rs:329
             at libcore/ops/function.rs:271
             at libcore/option.rs:404
             at libcore/iter/mod.rs:1347
             at librustc_data_structures/array_vec.rs:197
  18: rustc::ty::fold::TypeFoldable::fold_with
             at librustc_data_structures/accumulate_vec.rs:114
             at libcore/iter/iterator.rs:1372
             at librustc/ty/subst.rs:329
             at librustc/ty/fold.rs:55
  19: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with
             at librustc/ty/structural_impls.rs:993
  20: <rustc::ty::subst::SubstFolder<'a, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
             at librustc/ty/subst.rs:459
  21: <rustc_mir::interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M>>::eval_operand
             at librustc/ty/structural_impls.rs:1019
             at librustc/ty/subst.rs:378
             at librustc/ty/subst.rs:357
             at librustc_mir/interpret/eval_context.rs:289
             at librustc_mir/interpret/eval_context.rs:801
  22: rustc_mir::interpret::step::<impl rustc_mir::interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M>>::step
             at librustc_mir/interpret/terminator/mod.rs:69
             at librustc_mir/interpret/step.rs:107
             at librustc_mir/interpret/step.rs:43
  23: rustc_mir::interpret::const_eval::eval_body_and_ecx
             at librustc_mir/interpret/const_eval.rs:149
             at librustc_mir/interpret/const_eval.rs:102
  24: rustc_mir::interpret::const_eval::const_eval_provider
             at librustc_mir/interpret/const_eval.rs:494
  25: rustc::ty::maps::<impl rustc::ty::maps::queries::const_eval<'tcx>>::compute_result
             at librustc/ty/maps/plumbing.rs:396
  26: rustc::dep_graph::graph::DepGraph::with_task_impl
             at librustc/dep_graph/graph.rs:230
  27: rustc_errors::Handler::track_diagnostics
             at librustc/dep_graph/graph.rs:201
             at librustc/ty/maps/plumbing.rs:505
             at librustc_errors/lib.rs:601
  28: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
             at librustc/ty/maps/plumbing.rs:498
             at librustc/ty/maps/plumbing.rs:121
  29: rustc::ty::maps::<impl rustc::ty::maps::queries::const_eval<'tcx>>::force
             at librustc/ty/maps/plumbing.rs:497
  30: rustc::ty::maps::<impl rustc::ty::maps::queries::const_eval<'tcx>>::try_get
             at librustc/ty/maps/plumbing.rs:361
             at librustc/ty/maps/plumbing.rs:539
  31: rustc::ty::maps::TyCtxtAt::const_eval
             at librustc/ty/maps/plumbing.rs:578
  32: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::const_eval
             at librustc/ty/maps/plumbing.rs:571
  33: rustc_typeck::check::FnCtxt::check_expr_kind
             at librustc_typeck/check/mod.rs:4012
  34: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs
             at librustc_typeck/check/mod.rs:3550
  35: rustc_typeck::check::FnCtxt::check_block_with_expected
             at librustc_typeck/check/mod.rs:2820
             at librustc_typeck/check/mod.rs:4392
             at libcore/option.rs:404
             at librustc_typeck/check/mod.rs:4392
             at librustc_typeck/check/mod.rs:5090
             at librustc_typeck/check/mod.rs:4385
  36: rustc_typeck::check::FnCtxt::check_expr_kind
             at librustc_typeck/check/mod.rs:3933
  37: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs
             at librustc_typeck/check/mod.rs:3550
  38: rustc_typeck::check::FnCtxt::check_return_expr
             at librustc_typeck/check/mod.rs:2820
             at librustc_typeck/check/mod.rs:2814
             at librustc_typeck/check/mod.rs:2956
  39: rustc_typeck::check::check_fn
             at librustc_typeck/check/mod.rs:1058
  40: rustc::ty::context::tls::enter
             at librustc_typeck/check/mod.rs:856
             at librustc_typeck/check/mod.rs:618
             at librustc/infer/mod.rs:439
             at librustc/ty/context.rs:1681
             at libstd/thread/local.rs:290
             at libstd/thread/local.rs:244
             at librustc/ty/context.rs:1678
  41: rustc::infer::InferCtxtBuilder::enter
             at librustc/ty/context.rs:1489
             at librustc/infer/mod.rs:439
  42: rustc_typeck::check::typeck_tables_of
             at librustc_typeck/check/mod.rs:618
             at librustc_typeck/check/mod.rs:840
  43: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::compute_result
             at librustc/ty/maps/plumbing.rs:396
  44: rustc::dep_graph::graph::DepGraph::with_task_impl
             at librustc/dep_graph/graph.rs:230
  45: rustc_errors::Handler::track_diagnostics
             at librustc/dep_graph/graph.rs:201
             at librustc/ty/maps/plumbing.rs:505
             at librustc_errors/lib.rs:601
  46: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
             at librustc/ty/maps/plumbing.rs:498
             at librustc/ty/maps/plumbing.rs:121
  47: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::force
             at librustc/ty/maps/plumbing.rs:497
  48: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::try_get
             at librustc/ty/maps/plumbing.rs:361
             at librustc/ty/maps/plumbing.rs:539
  49: rustc::ty::maps::TyCtxtAt::typeck_tables_of
             at librustc/ty/maps/plumbing.rs:578
  50: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::ensure
             at librustc/ty/maps/plumbing.rs:571
             at librustc/ty/maps/plumbing.rs:390
  51: rustc_typeck::check::typeck_item_bodies
             at librustc_typeck/check/mod.rs:716
             at librustc/session/mod.rs:316
             at librustc_typeck/check/mod.rs:714
  52: rustc::dep_graph::graph::DepGraph::with_task_impl
             at librustc/dep_graph/graph.rs:230
  53: rustc_errors::Handler::track_diagnostics
             at librustc/dep_graph/graph.rs:201
             at librustc/ty/maps/plumbing.rs:505
             at librustc_errors/lib.rs:601
  54: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
             at librustc/ty/maps/plumbing.rs:498
             at librustc/ty/maps/plumbing.rs:121
  55: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::force
             at librustc/ty/maps/plumbing.rs:497
  56: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::try_get
             at librustc/ty/maps/plumbing.rs:361
             at librustc/ty/maps/plumbing.rs:539
  57: rustc::ty::maps::TyCtxtAt::typeck_item_bodies
             at librustc/ty/maps/plumbing.rs:578
  58: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_item_bodies
             at librustc/ty/maps/plumbing.rs:571
  59: rustc_typeck::check_crate
             at librustc_typeck/lib.rs:352
  60: rustc::ty::context::tls::enter_global
             at librustc_driver/driver.rs:1032
             at librustc/ty/context.rs:1681
             at libstd/thread/local.rs:290
             at libstd/thread/local.rs:244
             at librustc/ty/context.rs:1678
             at librustc/ty/context.rs:1665
             at libstd/thread/local.rs:290
             at libstd/thread/local.rs:244
             at librustc/ty/context.rs:1662
  61: rustc_driver::driver::compile_input
             at librustc/ty/context.rs:1220
             at librustc_driver/driver.rs:1007
             at librustc_driver/driver.rs:215
  62: rustc_driver::run_compiler
             at librustc_driver/lib.rs:520
