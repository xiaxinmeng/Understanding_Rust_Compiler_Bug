
stack backtrace:
   0: <std::time::SystemTimeError as core::fmt::Display>::fmt
   1: std::panicking::Location::line
   2: std::panicking::Location::line
   3: std::panicking::rust_panic_with_hook
   4: <unknown>
   5: rustc_typeck::check::FnCtxt::field_ty
   6: rustc_typeck::check::method::confirm::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::confirm_method
   7: rustc_typeck::check::method::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::lookup_method
   8: rustc_typeck::check::FnCtxt::check_struct_path
   9: rustc_typeck::check::FnCtxt::check_struct_path
  10: rustc_typeck::check::FnCtxt::check_stmt
  11: rustc_typeck::check::FnCtxt::check_block_no_value
  12: rustc_typeck::check::FnCtxt::check_block_no_value
  13: rustc_typeck::check::FnCtxt::check_struct_path
  14: rustc_typeck::check::FnCtxt::check_struct_path
  15: rustc_typeck::check::FnCtxt::impl_self_ty
  16: <rustc_typeck::check::GatherLocalsVisitor<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_pat
  17: rustc_typeck::check::provide
  18: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::try_get
  19: rustc::ty::maps::TyCtxtAt::typeck_tables_of
  20: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_tables_of
  21: rustc_typeck::check::check_item_bodies
  22: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::try_get
  23: rustc::ty::maps::TyCtxtAt::typeck_item_bodies
  24: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_item_bodies
  25: rustc_typeck::check_crate
  26: rustc_driver::driver::count_nodes
  27: rustc_driver::driver::count_nodes
  28: rustc_driver::driver::compile_input
  29: rustc_driver::run_compiler
  30: <unknown>
  31: _rust_maybe_catch_panic
  32: <rustc_driver::derive_registrar::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor<'v>>::visit_impl_item
  33: std::sys::imp::thread::Thread::new
  34: BaseThreadInitThunk

