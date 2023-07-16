Rust
thread 'rustc' panicked at 'region constraints already solved', /tmp/tmp.gJ7nOH7fpQ/rust/src/libcore/option.rs:874:4
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/libstd/sys_common/backtrace.rs:68
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/libstd/sys_common/backtrace.rs:57
   2: std::panicking::default_hook::{{closure}}
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/libstd/panicking.rs:381
   3: std::panicking::default_hook
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/libstd/panicking.rs:391
   4: std::panicking::rust_panic_with_hook
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/libstd/panicking.rs:577
   5: std::panicking::begin_panic
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/libstd/panicking.rs:538
   6: std::panicking::begin_panic_fmt
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/libstd/panicking.rs:522
   7: rust_begin_unwind
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/libstd/panicking.rs:498
   8: core::panicking::panic_fmt
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/libcore/panicking.rs:71
   9: core::option::expect_failed
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/libcore/option.rs:874
  10: rustc::infer::InferCtxt::start_snapshot
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/libcore/option.rs:302
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/infer/mod.rs:1537
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/libcore/cell.rs:1090
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/infer/mod.rs:1535
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/infer/mod.rs:790
  11: rustc::infer::InferCtxt::probe
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/infer/mod.rs:904
  12: rustc::traits::project::opt_normalize_projection_type
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/traits/project.rs:1002
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/traits/project.rs:807
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/traits/project.rs:541
  13: rustc::traits::project::normalize_projection_type
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/traits/project.rs:398
  14: <rustc::traits::project::AssociatedTypeNormalizer<'a, 'b, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/traits/project.rs:318
  15: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::fold_with
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/ty/structural_impls.rs:695
  16: rustc::traits::project::normalize
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/traits/project.rs:266
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/traits/project.rs:225
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/traits/project.rs:209
  17: rustc::traits::fully_normalize
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/traits/mod.rs:605
  18: rustc_mir::transform::nll::constraint_generation::ConstraintGeneration::add_drop_live_constraint
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc_mir/transform/nll/constraint_generation.rs:158
  19: rustc_mir::transform::nll::constraint_generation::ConstraintGeneration::add_liveness_constraints::{{closure}}
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc_mir/transform/nll/constraint_generation.rs:81
  20: rustc_mir::util::liveness::LivenessResult::simulate_block
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc_mir/util/liveness.rs:151
  21: rustc_mir::transform::nll::constraint_generation::generate_constraints
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc_mir/transform/nll/constraint_generation.rs:76
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc_mir/transform/nll/constraint_generation.rs:54
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc_mir/transform/nll/constraint_generation.rs:35
  22: rustc_mir::transform::nll::compute_regions
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc_mir/transform/nll/mod.rs:78
  23: rustc_mir::borrow_check::do_mir_borrowck
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc_mir/borrow_check.rs:116
  24: <std::thread::local::LocalKey<T>>::with
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc_mir/borrow_check.rs:65
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/infer/mod.rs:439
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/ty/context.rs:1453
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/libstd/thread/local.rs:377
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/libstd/thread/local.rs:288
  25: rustc::ty::context::tls::enter
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/ty/context.rs:1450
  26: rustc::ty::context::GlobalCtxt::enter_local
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/ty/context.rs:1261
  27: rustc::infer::InferCtxtBuilder::enter
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/infer/mod.rs:439
  28: rustc_mir::borrow_check::mir_borrowck
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc_mir/borrow_check.rs:63
  29: rustc::ty::maps::<impl rustc::ty::maps::queries::mir_borrowck<'tcx>>::compute_result   
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/ty/maps/plumbing.rs:382
  30: rustc::dep_graph::graph::DepGraph::with_task_impl
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/dep_graph/graph.rs:273
  31: rustc::dep_graph::graph::DepGraph::with_task
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/dep_graph/graph.rs:189
  32: rustc_errors::Handler::track_diagnostics
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/ty/maps/plumbing.rs:472
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc_errors/lib.rs:564
  33: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/ty/maps/plumbing.rs:465
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/ty/maps/plumbing.rs:120
  34: rustc::ty::maps::<impl rustc::ty::maps::queries::mir_borrowck<'tcx>>::force
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/ty/maps/plumbing.rs:464
  35: rustc::ty::maps::<impl rustc::ty::maps::queries::mir_borrowck<'tcx>>::try_get
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/ty/maps/plumbing.rs:300
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/ty/maps/plumbing.rs:506
  36: rustc::ty::maps::TyCtxtAt::mir_borrowck
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/ty/maps/plumbing.rs:545
  37: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::mir_borrowck
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/ty/maps/plumbing.rs:538
  38: rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::{{closure}}
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc_driver/driver.rs:1072
  39: rustc::util::common::time
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/util/common.rs:120
  40: <std::thread::local::LocalKey<T>>::with
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc_driver/driver.rs:1070
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/librustc/ty/context.rs:1453
             at /tmp/tmp.gJ7nOH7fpQ/rust/src/libstd/thread/local.rs:377
