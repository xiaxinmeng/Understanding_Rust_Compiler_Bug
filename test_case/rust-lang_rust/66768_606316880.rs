
error: internal compiler error: broken MIR in NodeId(9) (Terminator { source_info: SourceInfo { span: main.rs:9:26: 9:73, scope: scope1 }, kind: _4 = const FiniteElement::map_reference_coords(_5) -> bb1 }): call dest mismatch (Matrix<f64, DimU2, DimU1, <DefaultAllocator as Allocator<f64, DimU2>>::Buffer> <- Matrix<f64, DimU2, DimU1, ArrayStorage<_, _, _>>): Sorts(ExpectedFound { expected: <DefaultAllocator as Allocator<f64, DimU2>>::Buffer, found: ArrayStorage<_, _, _> })
 --> main.rs:9:26
  |
9 |     let _: Point2<f64> = material_surface_element.map_reference_coords().into();
  |                          ^^^^^^^^^^^^^^^^^^^^^^^^

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.21.0-nightly (7eeac1b81 2017-08-30) running on x86_64-pc-windows-msvc

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', src\librustc_errors\lib.rs:439:8
stack backtrace:
   0: _rdl_shrink_in_place
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
  22: rustc::ty::maps::<impl rustc::ty::maps::queries::coherent_trait<'tcx>>::force
  23: rustc::dep_graph::graph::DepGraph::in_task
  24: rustc::ty::maps::<impl rustc::ty::maps::queries::borrowck<'tcx>>::try_get
  25: rustc::ty::maps::TyCtxtAt::borrowck
  26: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::borrowck
  27: rustc_borrowck::borrowck::check_crate
  28: <rustc_driver::derive_registrar::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor<'v>>::visit_impl_item
  29: rustc_driver::driver::compile_input
  30: rustc_driver::run_compiler
  31: <unknown>
  32: _rust_maybe_catch_panic
  33: <rustc_driver::derive_registrar::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor<'v>>::visit_impl_item
  34: std::sys::imp::thread::Thread::new
  35: BaseThreadInitThunk



nightly-2017-08-31 finished with exit code Some(101).
