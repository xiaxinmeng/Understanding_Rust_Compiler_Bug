
paulg@debian ~/P/playrust> env RUST_BACKTRACE=1 cargo +test run --verbose
   Compiling playrust v0.1.0 (file:///home/paulg/Projets/playrust)
     Running `rustc --crate-name playrust src/main.rs --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=f6c25ba09712dbb2 -C extra-filename=-f6c25ba09712dbb2 --out-dir /home/paulg/Projets/playrust/target/debug/deps -L dependency=/home/paulg/Projets/playrust/target/debug/deps -C target-cpu=native`
error: internal compiler error: src/librustc_mir/borrow_check/nll/universal_regions.rs:805: cannot convert `ReScope(Node(ItemLocalId(57)))` to a region vid

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.24.0-dev running on x86_64-unknown-linux-gnu

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:502:8
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: std::panicking::rust_panic_with_hook
   5: std::panicking::begin_panic
   6: rustc_errors::Handler::bug
   7: <std::thread::local::LocalKey<T>>::with
   8: rustc::ty::context::tls::with_opt
   9: rustc::session::opt_span_bug_fmt
  10: rustc::session::bug_fmt
  11: rustc_mir::borrow_check::nll::universal_regions::UniversalRegionIndices::to_region_vid
  12: rustc_mir::borrow_check::nll::subtype_constraint_generation::generate
  13: rustc_mir::borrow_check::nll::compute_regions
  14: rustc_mir::borrow_check::do_mir_borrowck
  15: rustc::ty::context::tls::enter
  16: rustc::infer::InferCtxtBuilder::enter
  17: rustc_mir::borrow_check::mir_borrowck
  18: rustc::ty::maps::<impl rustc::ty::maps::queries::mir_borrowck<'tcx>>::compute_result
  19: rustc::dep_graph::graph::DepGraph::with_task_impl
  20: rustc_errors::Handler::track_diagnostics
  21: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
  22: rustc::ty::maps::<impl rustc::ty::maps::queries::mir_borrowck<'tcx>>::force
  23: rustc::ty::maps::<impl rustc::ty::maps::queries::mir_borrowck<'tcx>>::try_get
  24: rustc::ty::maps::TyCtxtAt::mir_borrowck
  25: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::mir_borrowck
  26: rustc_mir::transform::optimized_mir
  27: rustc::ty::maps::<impl rustc::ty::maps::queries::optimized_mir<'tcx>>::compute_result
  28: rustc::dep_graph::graph::DepGraph::with_task_impl
  29: rustc_errors::Handler::track_diagnostics
  30: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
  31: rustc::ty::maps::<impl rustc::ty::maps::queries::optimized_mir<'tcx>>::force
  32: rustc::ty::maps::<impl rustc::ty::maps::queries::optimized_mir<'tcx>>::try_get
  33: rustc::ty::maps::TyCtxtAt::optimized_mir
  34: rustc::ty::sty::ClosureSubsts::field_tys
  35: rustc_mir::borrow_check::nll::type_check::TypeChecker::typeck_mir
  36: rustc_mir::borrow_check::nll::type_check::type_check_internal
  37: rustc::ty::context::tls::enter
  38: <rustc_mir::borrow_check::nll::type_check::TypeckMir as rustc_mir::transform::MirPass>::run_pass
  39: rustc_mir::transform::mir_const::{{closure}}
  40: rustc_mir::transform::mir_const
  41: rustc::ty::maps::<impl rustc::ty::maps::queries::mir_const<'tcx>>::compute_result
  42: rustc::dep_graph::graph::DepGraph::with_task_impl
  43: rustc_errors::Handler::track_diagnostics
  44: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
  45: rustc::ty::maps::<impl rustc::ty::maps::queries::mir_const<'tcx>>::force
  46: rustc::ty::maps::<impl rustc::ty::maps::queries::mir_const<'tcx>>::try_get
  47: rustc::ty::maps::TyCtxtAt::mir_const
  48: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::mir_const
  49: rustc_mir::transform::mir_validated
  50: rustc::ty::maps::<impl rustc::ty::maps::queries::mir_validated<'tcx>>::compute_result
  51: rustc::dep_graph::graph::DepGraph::with_task_impl
  52: rustc_errors::Handler::track_diagnostics
  53: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
  54: rustc::ty::maps::<impl rustc::ty::maps::queries::mir_validated<'tcx>>::force
  55: rustc::ty::maps::<impl rustc::ty::maps::queries::mir_validated<'tcx>>::try_get
  56: rustc::ty::maps::TyCtxtAt::mir_validated
  57: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::mir_validated
  58: rustc_borrowck::borrowck::borrowck
  59: rustc::dep_graph::graph::DepGraph::with_task_impl
  60: rustc_errors::Handler::track_diagnostics
  61: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
  62: rustc::ty::maps::<impl rustc::ty::maps::queries::borrowck<'tcx>>::force
  63: rustc::ty::maps::<impl rustc::ty::maps::queries::borrowck<'tcx>>::try_get
  64: rustc::ty::maps::TyCtxtAt::borrowck
  65: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::borrowck
  66: rustc_borrowck::borrowck::check_crate
  67: <std::thread::local::LocalKey<T>>::with
  68: <std::thread::local::LocalKey<T>>::with
  69: rustc::ty::context::TyCtxt::create_and_enter
  70: rustc_driver::driver::compile_input
  71: rustc_driver::run_compiler

error: Could not compile `playrust`.

Caused by:
  process didn't exit successfully: `rustc --crate-name playrust src/main.rs --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=f6c25ba09712dbb2 -C extra-filename=-f6c25ba09712dbb2 --out-dir /home/paulg/Projets/playrust/target/debug/deps -L dependency=/home/paulg/Projets/playrust/target/debug/deps -C target-cpu=native` (exit code: 101)
