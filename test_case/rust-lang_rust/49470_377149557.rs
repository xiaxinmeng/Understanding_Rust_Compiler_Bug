
$ RUST_BACKTRACE=1 rustc 1.rs
thread 'rustc' panicked at 'assertion failed: !substs.has_erasable_regions()', librustc_trans_utils/symbol_names.rs:169:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: core::ops::function::Fn::call
   5: std::panicking::rust_panic_with_hook
   6: std::panicking::begin_panic
   7: rustc::util::common::record_time
   8: rustc_trans_utils::symbol_names::symbol_name
   9: rustc::ty::maps::<impl rustc::ty::maps::queries::symbol_name<'tcx>>::compute_result
  10: rustc::dep_graph::graph::DepGraph::with_task_impl
  11: rustc_errors::Handler::track_diagnostics
  12: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
  13: rustc::ty::maps::<impl rustc::ty::maps::queries::symbol_name<'tcx>>::force
  14: rustc::ty::maps::<impl rustc::ty::maps::queries::symbol_name<'tcx>>::try_get
  15: rustc::ty::maps::TyCtxtAt::symbol_name
  16: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::symbol_name
  17: rustc_mir::monomorphize::item::MonoItemExt::symbol_name
  18: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  19: rustc_mir::monomorphize::assert_symbols_are_distinct
  20: rustc_trans::base::collect_and_partition_translation_items
  21: rustc::dep_graph::graph::DepGraph::with_task_impl
  22: rustc_errors::Handler::track_diagnostics
  23: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
  24: rustc::ty::maps::<impl rustc::ty::maps::queries::collect_and_partition_translation_items<'tcx>>::force
  25: rustc::ty::maps::<impl rustc::ty::maps::queries::collect_and_partition_translation_items<'tcx>>::try_get
  26: rustc::ty::maps::TyCtxtAt::collect_and_partition_translation_items
  27: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::collect_and_partition_translation_items
  28: rustc_trans::base::trans_crate
  29: <rustc_trans::LlvmTransCrate as rustc_trans_utils::trans_crate::TransCrate>::trans_crate
  30: rustc::util::common::time
  31: rustc_driver::driver::phase_4_translate_to_llvm
  32: rustc_driver::driver::compile_input::{{closure}}
  33: <std::thread::local::LocalKey<T>>::with
  34: <std::thread::local::LocalKey<T>>::with
  35: rustc::ty::context::TyCtxt::create_and_enter
  36: rustc_driver::driver::compile_input
  37: rustc_driver::run_compiler_impl
  38: syntax::with_globals

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.26.0-nightly (9c9424de5 2018-03-27) running on x86_64-apple-darwin
