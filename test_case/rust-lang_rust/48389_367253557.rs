
thread 'rustc' panicked at 'assertion failed: !substs.has_erasable_regions()', librustc_trans_utils/symbol_names.rs:192:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: std::panicking::rust_panic_with_hook
   5: std::panicking::begin_panic
   6: rustc::util::common::record_time
   7: rustc_trans_utils::symbol_names::symbol_name
   8: rustc::ty::maps::<impl rustc::ty::maps::queries::symbol_name<'tcx>>::compute_result
   9: rustc::dep_graph::graph::DepGraph::with_task_impl
  10: rustc_errors::Handler::track_diagnostics
  11: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
  12: rustc::ty::maps::<impl rustc::ty::maps::queries::symbol_name<'tcx>>::force
  13: rustc::ty::maps::<impl rustc::ty::maps::queries::symbol_name<'tcx>>::try_get
  14: rustc::ty::maps::TyCtxtAt::symbol_name
  15: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::symbol_name
  16: rustc_mir::monomorphize::item::MonoItemExt::symbol_name
  17: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  18: rustc_mir::monomorphize::assert_symbols_are_distinct
  19: rustc_trans::base::collect_and_partition_translation_items
  20: rustc::dep_graph::graph::DepGraph::with_task_impl
  21: rustc_errors::Handler::track_diagnostics
  22: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
  23: rustc::ty::maps::<impl rustc::ty::maps::queries::collect_and_partition_translation_items<'tcx>>::force
  24: rustc::ty::maps::<impl rustc::ty::maps::queries::collect_and_partition_translation_items<'tcx>>::try_get
  25: rustc::ty::maps::TyCtxtAt::collect_and_partition_translation_items
  26: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::collect_and_partition_translation_items
  27: <rustc_trans::LlvmTransCrate as rustc_trans_utils::trans_crate::TransCrate>::trans_crate
  28: rustc::util::common::time
  29: rustc_driver::driver::phase_4_translate_to_llvm
  30: rustc_driver::driver::compile_input::{{closure}}
  31: <std::thread::local::LocalKey<T>>::with
  32: <std::thread::local::LocalKey<T>>::with
  33: rustc::ty::context::TyCtxt::create_and_enter
  34: rustc_driver::driver::compile_input
  35: rustc_driver::run_compiler
