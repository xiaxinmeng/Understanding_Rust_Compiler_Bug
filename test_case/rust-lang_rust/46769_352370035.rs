
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at /checkout/src/libstd/sys_common/backtrace.rs:68
             at /checkout/src/libstd/sys_common/backtrace.rs:57
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/panicking.rs:381
   3: std::panicking::default_hook
             at /checkout/src/libstd/panicking.rs:391
   4: std::panicking::rust_panic_with_hook
             at /checkout/src/libstd/panicking.rs:577
   5: std::panicking::begin_panic
             at /checkout/src/libstd/panicking.rs:538
   6: std::panicking::begin_panic_fmt
             at /checkout/src/libstd/panicking.rs:522
   7: rustc_trans::type_::Type::padding_filler
   8: rustc_trans::type_of::struct_llfields
   9: <rustc::ty::layout::TyLayout<'tcx> as rustc_trans::type_of::LayoutLlvmExt<'tcx>>::llvm_type
  10: rustc_trans::abi::FnType::llvm_type
  11: rustc_trans::declare::declare_fn
  12: rustc_trans::trans_item::predefine_fn
  13: rustc_trans::trans_item::TransItemExt::predefine
  14: rustc_trans::base::compile_codegen_unit
  15: rustc::ty::maps::<impl rustc::ty::maps::queries::compile_codegen_unit<'tcx>>::compute_result
  16: rustc::dep_graph::graph::DepGraph::with_task_impl
  17: rustc_errors::Handler::track_diagnostics
  18: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
  19: rustc::ty::maps::<impl rustc::ty::maps::queries::compile_codegen_unit<'tcx>>::force
  20: rustc::ty::maps::<impl rustc::ty::maps::queries::compile_codegen_unit<'tcx>>::try_get
  21: rustc::ty::maps::TyCtxtAt::compile_codegen_unit
  22: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::compile_codegen_unit
  23: rustc_trans::base::trans_crate
  24: <rustc_trans::LlvmTransCrate as rustc_trans_utils::trans_crate::TransCrate>::trans_crate
  25: rustc_driver::driver::phase_4_translate_to_llvm
  26: rustc_driver::driver::compile_input::{{closure}}
  27: <std::thread::local::LocalKey<T>>::with
  28: <std::thread::local::LocalKey<T>>::with
  29: rustc::ty::context::TyCtxt::create_and_enter
  30: rustc_driver::driver::compile_input
  31: rustc_driver::run_compiler
