
error: internal compiler error: src\lib.rs:1: librustc\ty\subst.rs:481: Type parameter `P/#3` (P/3) out of range when substituting (root type=Some(highlvl::HighLvlMsg<P, V, B>)) substs=[_, _, _]
thread 'rustc' panicked at 'Box<Any>', librustc_errors\lib.rs:482:9
stack backtrace:
   0: _rdl_shrink_in_place
   1: std::panicking::take_hook
   2: std::panicking::take_hook
   3: <unknown>
   4: std::panicking::rust_panic_with_hook
   5: <unknown>
   6: <unknown>
   7: rustc::session::bug_fmt
   8: rustc::ty::context::tls::span_debug
   9: rustc::ty::context::tls::span_debug
  10: <unknown>
  11: <unknown>
  12: rustc::ty::context::tls::span_debug
  13: rustc::ty::context::tls::span_debug
  14: rustc::session::bug_fmt
  15: rustc::session::bug_fmt
  16: <rustc::ty::subst::SubstFolder<'a, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  17: rustc::ty::subst::<impl rustc::ty::Slice<rustc::ty::subst::Kind<'tcx>>>::truncate_to
  18: <rustc::ty::subst::SubstFolder<'a, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  19: rustc::traits::project::normalize_projection_type
  20: rustc::traits::project::normalize_projection_type
  21: rustc::traits::project::normalize_projection_type
  22: rustc::traits::project::normalize_projection_type
  23: <rustc::traits::project::AssociatedTypeNormalizer<'a, 'b, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  24: rustc::ty::subst::<impl rustc::ty::Slice<rustc::ty::subst::Kind<'tcx>>>::truncate_to
  25: rustc::ty::structural_impls::<impl rustc::ty::context::Lift<'tcx> for rustc::ty::layout::LayoutError<'a>>::lift_to_tcx
  26: rustc::traits::project::poly_project_and_unify_type
  27: rustc::ty::wf::predicate_obligations
  28: <unknown>
  29: <unknown>
  30: rustc::ty::wf::trait_obligations
  31: <rustc_typeck::check::upvar::InferBorrowKind<'a, 'gcx, 'tcx> as rustc::middle::expr_use_visitor::Delegate<'tcx>>::mutate
  32: <rustc_typeck::check::wfcheck::CheckTypeWellFormedVisitor<'a, 'tcx> as rustc::hir::intravisit::Visitor<'v>>::visit_item
  33: rustc_typeck::check_crate
  34: rustc_typeck::check_crate
  35: <rustc_driver::derive_registrar::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor<'v>>::visit_impl_item
  36: rustc_driver::driver::compile_input
  37: rustc_driver::run_compiler
  38: <unknown>
  39: _rust_maybe_catch_panic
  40: <rustc_driver::derive_registrar::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor<'v>>::visit_impl_item
  41: std::sys::windows::thread::Thread::new
  42: BaseThreadInitThunk
  43: RtlUserThreadStart

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.26.0-nightly (2789b067d 2018-03-06) running on x86_64-pc-windows-msvc

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden
