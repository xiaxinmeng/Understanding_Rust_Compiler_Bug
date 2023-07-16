
   Compiling voodoo v0.0.1 (file:///C:/src/voodoo)
error: internal compiler error: broken MIR (Terminator { source_info: SourceInfo { span: src\physical_device.rs:86:58: 86:65, scope: scope3 }, kind: _51 = const std::ops::DerefMut::deref_mut(_52) -> [return: bb29, unwind: bb30] }): call dest mismatch (&mut [vks_::khr_surface::VkSurfaceFormatKHR] <- &mut [_]): Sorts(ExpectedFound { expected: vks_::khr_surface::VkSurfaceFormatKHR, found: structs::SurfaceFormatKhr })
  --> src\physical_device.rs:86:58
   |
86 |                     surface.handle(), &mut format_count, formats.as_mut_ptr() as *mut vks::VkSurfaceFormatKHR);
   |                                                          ^^^^^^^

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.20.0 (f3d6973f4 2017-08-27) running on x86_64-pc-windows-msvc

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', src\librustc_errors\lib.rs:437:8
stack backtrace:
   0: _rdl_grow_in_place
   1: std::panicking::Location::column
   2: std::panicking::Location::column
   3: std::panicking::rust_panic_with_hook
   4: <unknown>
   5: <unknown>
   6: <rustc_mir::transform::type_check::TypeVerifier<'a, 'b, 'gcx, 'tcx> as rustc::mir::visit::Visitor<'tcx>>::visit_mir
   7: <rustc_mir::transform::type_check::TypeckMir as rustc::mir::transform::MirPass>::run_pass
   8: <rustc_mir::transform::mir_keys::GatherCtors<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_variant_data
   9: <rustc_mir::transform::mir_keys::GatherCtors<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_variant_data
  10: rustc::ty::maps::<impl rustc::ty::maps::queries::mir_const_qualif<'tcx>>::force
  11: rustc::dep_graph::graph::DepGraph::in_task
  12: rustc::ty::maps::<impl rustc::ty::maps::queries::mir_const<'tcx>>::try_get
  13: rustc::ty::maps::TyCtxtAt::mir_const
  14: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::mir_const
  15: <rustc_mir::transform::mir_keys::GatherCtors<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_variant_data
  16: rustc::ty::maps::<impl rustc::ty::maps::queries::mir_const<'tcx>>::force
  17: rustc::dep_graph::graph::DepGraph::in_task
  18: rustc::ty::maps::<impl rustc::ty::maps::queries::mir_validated<'tcx>>::try_get
  19: rustc::ty::maps::TyCtxtAt::mir_validated
  20: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::mir_validated
  21: rustc_borrowck::borrowck::provide
  22: rustc::dep_graph::graph::DepGraph::in_task
  23: rustc::ty::maps::<impl rustc::ty::maps::queries::borrowck<'tcx>>::try_get
  24: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::borrowck
  25: rustc_borrowck::borrowck::check_crate
  26: rustc_driver::driver::count_nodes
  27: rustc_driver::driver::count_nodes
  28: rustc_driver::driver::compile_input
  29: rustc_driver::run_compiler
  30: <unknown>
  31: _rust_maybe_catch_panic
  32: <rustc_driver::derive_registrar::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor<'v>>::visit_trait_item
  33: std::sys::imp::thread::Thread::new
  34: BaseThreadInitThunk

error: Could not compile `voodoo`.
