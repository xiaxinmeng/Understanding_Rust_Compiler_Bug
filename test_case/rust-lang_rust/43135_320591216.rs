
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:490:8
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
  (panic handler, omitted)
  10: rustc_trans::collector::should_trans_locally
  11: rustc_trans::collector::visit_instance_use
  12: <rustc_trans::collector::MirNeighborCollector<'a, 'tcx> as rustc::mir::visit::Visitor<'tcx>>::visit_terminator_kind
  13: rustc::mir::visit::Visitor::visit_mir
  14: rustc_trans::collector::collect_items_rec
  (... recursion, omitted ...)
  42: rustc_trans::collector::collect_items_rec
  43: rustc_trans::base::collect_and_partition_translation_items::{{closure}}
  44: rustc_trans::base::trans_crate
  45: rustc_driver::driver::phase_4_translate_to_llvm
  46: rustc_driver::driver::compile_input::{{closure}}
  47: rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}
  48: rustc_driver::driver::phase_3_run_analysis_passes
  49: rustc_driver::driver::compile_input
  50: rustc_driver::run_compiler
